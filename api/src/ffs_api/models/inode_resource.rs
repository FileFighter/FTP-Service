use super::user_resource::UserResource;

#[derive(Debug, Serialize, Deserialize)]
pub struct InodeResource {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: u64,
    #[serde(rename = "lastUpdatedBy")]
    pub last_updated_by: UserResource,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "size")]
    pub size: u64,
}
