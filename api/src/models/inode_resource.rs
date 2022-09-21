use super::user_resource::UserResource;

#[derive(Debug, Serialize, Deserialize)]
pub struct InodeResource {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "lastUpdated")]
    last_updated: i64,
    #[serde(rename = "lastUpdatedBy")]
    last_updated_by: UserResource,
    #[serde(rename = "mimeType")]
    mime_type: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "size")]
    size: i64,
}
