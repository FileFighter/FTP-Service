#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResource {
    #[serde(rename = "mimeType")]
    mime_type: String,
    #[serde(rename = "parentPath")]
    parent_path: String,
    #[serde(rename = "relativePath")]
    relative_path: String,
    #[serde(rename = "size")]
    size: i64,
}
