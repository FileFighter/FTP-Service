#[derive(Debug, Serialize, Deserialize)]
pub struct FolderCreationResource {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "parentPath")]
    parent_path: String,
}
