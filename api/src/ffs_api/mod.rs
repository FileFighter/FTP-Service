use reqwest::Error;

pub mod endpoints;
pub mod models;

#[derive(Debug)]
pub struct ApiConfig {
    pub fss_base_url: String,
    pub fhs_base_url: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Request failed with error {0}")]
    ReqwestError(reqwest::Error),
    #[error("Response was malformed: {0}")]
    ResponseMalformed(String),
}

pub type Result<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(reqwest_error: Error) -> Self {
        ApiError::ReqwestError(reqwest_error)
    }
}
