#[derive(Debug, Serialize, Deserialize)]
pub struct RenameResource {
    #[serde(rename = "newName")]
    pub new_name: String,
    #[serde(rename = "path")]
    pub path: String,
}
