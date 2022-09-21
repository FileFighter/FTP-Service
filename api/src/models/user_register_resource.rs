#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterResource {
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "privileges")]
    privileges: String,
    #[serde(rename = "username")]
    username: String,
}
