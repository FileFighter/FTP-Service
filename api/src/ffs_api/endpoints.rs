use super::{ApiConfig, ApiError, Result};
use reqwest::StatusCode;
use tracing::debug;

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
