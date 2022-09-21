#[derive(Debug, Serialize, Deserialize)]
pub struct MoveResource {
    #[serde(rename = "newPath")]
    new_path: String,
    #[serde(rename = "path")]
    path: String,
}
