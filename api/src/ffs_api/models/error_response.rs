#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "status")]
    pub status: String,
}
