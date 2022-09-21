#[derive(Debug, Serialize, Deserialize)]
pub struct PreflightResponseResource {
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "result")]
    result: String,
}
