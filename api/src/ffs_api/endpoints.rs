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
    header::{HeaderMap, HeaderValue, ACCEPT},
    multipart, Response, StatusCode,
};
use serde::de::DeserializeOwned;
use std::path::Path;
use tokio::io::AsyncRead;
use tracing::debug;

// Compatibility trait lets us call `compat()` on a futures::io::AsyncRead
// to convert it into a tokio::io::AsyncRead.
use tokio_util::compat::FuturesAsyncReadCompatExt;

// Lets us call into_async_read() to convert a futures::stream::Stream into a
// futures::io::AsyncRead.
use futures::stream::TryStreamExt;

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
    let params = [("token", token)];
    let url = reqwest::Url::parse_with_params(&url, &params).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-FF-PARENT-PATH",
        HeaderValue::from_str(parent_path.to_str().unwrap()).unwrap(),
    );
    headers.insert(
        "X-FF-RELATIVE-PATH",
        HeaderValue::from_str(new_name).unwrap(),
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let some_file = multipart::Part::text("todo")
        .file_name("file")
        .mime_str("text/plain")?;
    let form = multipart::Form::new().part("file", some_file);

    let response = reqwest::Client::new()
        .post(url)
        .multipart(form)
        .headers(headers)
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn download_file<ByteStream>(
    api_config: &ApiConfig,
    token: &str,
    path: &Path,
) -> Result<Box<dyn AsyncRead + Send + Sync + Unpin>> {
    // inspired by https://github.com/benkay86/async-applied/blob/master/reqwest-tokio-compat/src/main.rs
    // but not working
    let url = format!(
        "{}/download{}",
        api_config.fhs_base_url,
        path.to_str().unwrap()
    );
    let params = [("token", token)];
    let url = reqwest::Url::parse_with_params(&url, &params).unwrap();

    let download = reqwest::get(url).await?.error_for_status()?;

    let download = download.bytes_stream();

    // Convert the stream into an futures::io::AsyncRead.
    // We must first convert the reqwest::Error into an futures::io::Error.
    let download = download
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read();

    // Convert the futures::io::AsyncRead into a tokio::io::AsyncRead.
    let download = download.compat();

    Ok(Box::new(download.get_mut()))
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
            error_response.status.unwrap_or("unkown".to_string()),
            error_response.message
        )))
    }
}
