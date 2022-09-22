#[derive(Debug, Serialize, Deserialize)]
pub struct UserResource {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "privileges")]
    pub privileges: String,
    #[serde(rename = "username")]
    pub username: String,
}
