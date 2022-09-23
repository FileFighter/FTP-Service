use crate::ffs_api::models::error_response::ErrorResponse;

use super::{
    models::{
        contents_resource::ContentsResource, folder_creation_resource::FolderCreationResource,
        inode_resource::InodeResource, user_resource::UserResource,
    },
    ApiConfig, ApiError, Result,
};
use reqwest::StatusCode;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

pub async fn get_token_for_user(
    api_config: &ApiConfig,
    username: &str,
    password: &str,
) -> Result<String> {
    let url = format!("{}/user/authenticate", api_config.base_url);
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
            .ok_or(ApiError::ResponseMalformed(
                "Could not find cookie in response".to_owned(),
            ))?
            .value()
            .to_owned()),
        code => Err(ApiError::ResponseMalformed(format!(
            "Response Code was {}, but expected 201",
            code
        ))),
    }
}

pub async fn get_user_info(api_config: &ApiConfig, token: &str) -> Result<UserResource> {
    let url = format!("{}/user/info", api_config.base_url);

    debug!("Getting user info with token '{}'", token);

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await?;

    Ok(response.json().await?)
}

pub async fn get_contents_of_folder(
    api_config: &ApiConfig,
    token: &str,
    path: PathBuf,
) -> Result<ContentsResource> {
    let url = format!("{}/filesystem/contents", api_config.base_url);

    debug!("Authenticating with token '{}'", token);

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .header("X-FF-PATH", path.to_str().unwrap())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        _ => {
            let error_response = response.json::<ErrorResponse>().await?;
            Err(ApiError::ResponseMalformed(format!(
                "Error response with code '{}' and reason '{}'.",
                error_response.status, error_response.message
            )))
        }
    }
}

pub async fn create_directory(
    api_config: &ApiConfig,
    token: &str,
    parent_path: &Path,
    name: &str,
) -> Result<InodeResource> {
    let url = format!("{}/filesystem/folder/create", api_config.base_url);

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

    match response.status() {
        StatusCode::CREATED => Ok(response.json().await?),
        _ => {
            let error_response = response.json::<ErrorResponse>().await?;
            Err(ApiError::ResponseMalformed(format!(
                "Error response with code '{}' and reason '{}'.",
                error_response.status, error_response.message
            )))
        }
    }
}
