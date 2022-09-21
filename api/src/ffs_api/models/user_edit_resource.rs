#[derive(Debug, Serialize, Deserialize)]
pub struct UserEditResource {
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "username")]
    username: String,
}
