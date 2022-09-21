#[derive(Debug, Serialize, Deserialize)]
pub struct RenameResource {
    #[serde(rename = "newName")]
    new_name: String,
    #[serde(rename = "path")]
    path: String,
}
