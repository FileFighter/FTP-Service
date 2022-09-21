#[derive(Debug, Serialize, Deserialize)]
pub struct PreflightRequestResource {
    #[serde(rename = "parentPath")]
    parent_path: String,
    #[serde(rename = "relativePaths")]
    relative_paths: Vec<String>,
}
