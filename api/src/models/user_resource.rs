#[derive(Debug, Serialize, Deserialize)]
pub struct UserResource {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "privileges")]
    privileges: String,
    #[serde(rename = "username")]
    username: String,
}
