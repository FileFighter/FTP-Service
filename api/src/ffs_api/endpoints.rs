use crate::ffs_api::models::{
    error_response::ErrorResponse, move_resource::MoveResource, rename_resource::RenameResource,
};

use super::{
    models::{
        contents_resource::ContentsResource, folder_creation_resource::FolderCreationResource,
        inode_resource::InodeResource, user_resource::UserResource,
    },
    ApiConfig, ApiError, Result,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response, StatusCode,
};
use serde::de::DeserializeOwned;
use std::path::Path;
use tokio::io::AsyncRead;
use tracing::debug;

pub async fn get_token_for_user(
    api_config: &ApiConfig,
    username: &str,
    password: &str,
) -> Result<String> {
    let url = format!("{}/user/authenticate", api_config.fss_base_url);
    let password = sha256::digest(format!("{}FileFighterWithSomeSalt", password)).to_uppercase();

    debug!("Authenticating with password '{}'", password);

    let response = reqwest::Client::builder()
        .cookie_store(true)
        .build()?
        .post(url)
        .basic_auth(username, Some(password))
        .send()
        .await?;

    match response.status() {
        StatusCode::CREATED => Ok(response
            .cookies()
            .find(|c| c.name() == "token")
            .ok_or_else(|| {
                ApiError::ResponseMalformed("Could not find cookie in response".to_owned())
            })?
            .value()
            .to_owned()),
        code => Err(ApiError::ResponseMalformed(format!(
            "Response Code was {}, but expected 201",
            code
        ))),
    }
}

pub async fn get_user_info(api_config: &ApiConfig, token: &str) -> Result<UserResource> {
    let url = format!("{}/user/info", api_config.fss_base_url);

    debug!("Getting user info with token '{}'", token);

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn get_inode(api_config: &ApiConfig, path: &Path, token: &str) -> Result<InodeResource> {
    let url = format!("{}/filesystem/info", api_config.fss_base_url);

    debug!("Getting inode by path '{}'", path.display());

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .header("X-FF-PATH", path.to_str().unwrap())
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn get_contents_of_folder(
    api_config: &ApiConfig,
    token: &str,
    path: &Path,
) -> Result<ContentsResource> {
    let url = format!("{}/filesystem/contents", api_config.fss_base_url);

    debug!("Authenticating with token '{}'", token);

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .header("X-FF-PATH", path.to_str().unwrap())
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn create_directory(
    api_config: &ApiConfig,
    token: &str,
    parent_path: &Path,
    name: &str,
) -> Result<InodeResource> {
    let url = format!("{}/filesystem/folder/create", api_config.fss_base_url);

    debug!("Authenticating with token '{}'", token);

    let body = FolderCreationResource {
        name: name.to_owned(),
        parent_path: parent_path.to_str().unwrap().to_owned(),
    };

    let response = reqwest::Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    transform_response(response, StatusCode::CREATED).await
}

pub async fn rename_inode(
    api_config: &ApiConfig,
    token: &str,
    parent_path: &Path,
    new_name: &str,
) -> Result<InodeResource> {
    let url = format!("{}/filesystem/rename", api_config.fss_base_url);

    debug!("Authenticating with token '{}'", token);

    let body = RenameResource {
        path: parent_path.to_str().unwrap().to_owned(),
        new_name: new_name.to_owned(),
    };

    let response = reqwest::Client::new()
        .put(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn move_inode(
    api_config: &ApiConfig,
    token: &str,
    parent_path: &Path,
    new_path: &Path,
) -> Result<InodeResource> {
    let url = format!("{}/filesystem/move", api_config.fss_base_url);

    debug!("Authenticating with token '{}'", token);

    let body = MoveResource {
        path: parent_path.to_str().unwrap().to_owned(),
        new_path: new_path.to_str().unwrap().to_owned(),
    };

    let response = reqwest::Client::new()
        .put(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn delete_inode(
    api_config: &ApiConfig,
    token: &str,
    path: &Path,
) -> Result<Vec<InodeResource>> {
    let url = format!(
        "{}/delete{}",
        api_config.fhs_base_url,
        path.to_str().unwrap()
    );
    let params = [("token", token)];

    let url = reqwest::Url::parse_with_params(&url, &params).unwrap();
    let response = reqwest::Client::new().delete(url).send().await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn upload_file<ByteStream>(
    api_config: &ApiConfig,
    token: &str,
    parent_path: &Path,
    new_name: &str,
    bytes: ByteStream,
) -> Result<Vec<InodeResource>>
where
    ByteStream: AsyncRead + Send + Sync + 'static + Unpin,
{
    let url = format!("{}/upload", api_config.fhs_base_url);
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-FF-PARENT-PATH",
        HeaderValue::from_str(parent_path.to_str().unwrap()).unwrap(),
    );
    headers.insert(
        "X-FF-RELATIVE-PATH",
        HeaderValue::from_str(new_name).unwrap(),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("multipart/form-data"),
    );
    let form = [("file", "todo" /* todo: should put bytes*/)];

    let response = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .form(&form)
        .send()
        .await?;
    todo!()
}

async fn transform_response<T>(response: Response, expected_status: StatusCode) -> Result<T>
where
    T: DeserializeOwned,
{
    if expected_status == response.status() {
        Ok(response.json().await?)
    } else {
        let error_response = response.json::<ErrorResponse>().await?;
        Err(ApiError::ResponseMalformed(format!(
            "Error response with code '{}' and reason '{}'.",
            error_response.status, error_response.message
        )))
    }
}
