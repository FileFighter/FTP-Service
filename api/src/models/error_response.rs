#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "status")]
    status: String,
}
