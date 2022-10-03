use crate::ffs_api::models::{
    error_response::ErrorResponse, inode_timestamp_update_ressource::InodeTimestampUpdateRessource,
    move_resource::MoveResource, rename_resource::RenameResource,
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
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::debug;

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

// IDEA: fix usage of relative paths to uploaded files. (When to create parent folder and when not)
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
    let stream = ReaderStream::new(bytes);

    let url = format!("{}/upload", api_config.fhs_base_url);
    let params = [("token", token)];
    let url = reqwest::Url::parse_with_params(&url, &params).unwrap();

    // set required headers
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

    // Stream the content as multipart stream
    let some_file = multipart::Part::stream(reqwest::Body::wrap_stream(stream))
        .file_name("file")
        .mime_str(
            new_mime_guess::from_path(new_name)
                .first_or_octet_stream()
                .as_ref(),
        )?;
    let form = multipart::Form::new().part("file", some_file);

    let response = reqwest::Client::new()
        .post(url)
        .multipart(form)
        .headers(headers)
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
}

pub async fn download_file(
    api_config: &ApiConfig,
    token: &str,
    path: &Path,
) -> Result<Box<dyn AsyncRead + Send + Sync + Unpin>> {
    // inspired by https://github.com/benkay86/async-applied/blob/master/reqwest-tokio-compat/src/main.rs
    let url = format!(
        "{}/download{}",
        api_config.fhs_base_url,
        path.to_str().unwrap()
    );
    let params = [("token", token)];
    let url = reqwest::Url::parse_with_params(&url, &params).unwrap();

    // get the content as stream and map the error so tokio can use `from` on it
    let download = reqwest::get(url).await?.error_for_status()?;
    let download = download
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e));

    // build a stream reader which allows us to use async read on a stream.
    let stream_reader = StreamReader::new(download);
    Ok(Box::new(stream_reader))
}

pub async fn set_last_modified_of_inode(
    api_config: &ApiConfig,
    token: &str,
    path: &Path,
    last_modified: i64,
) -> Result<InodeResource> {
    let url = format!("{}/filesystem/timestamp", api_config.fss_base_url);

    debug!("Authenticating with token '{}'", token);

    let body = InodeTimestampUpdateRessource {
        path: path.to_str().unwrap().to_owned(),
        timestamp: last_modified,
    };

    let response = reqwest::Client::new()
        .put(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    transform_response(response, StatusCode::OK).await
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
            error_response
                .status
                .unwrap_or_else(|| "unkown".to_string()),
            error_response.message
        )))
    }
}
