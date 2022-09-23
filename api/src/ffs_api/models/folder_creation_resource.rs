#[derive(Debug, Serialize, Deserialize)]
pub struct FolderCreationResource {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentPath")]
    pub parent_path: String,
}
