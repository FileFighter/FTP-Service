#[derive(Debug, Serialize, Deserialize)]
pub struct MoveResource {
    #[serde(rename = "newPath")]
    pub new_path: String,
    #[serde(rename = "path")]
    pub path: String,
}
