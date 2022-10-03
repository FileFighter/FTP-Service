#[derive(Debug, Serialize, Deserialize)]
pub struct InodeTimestampUpdateRessource {
    pub path: String,
    pub timestamp: i64,
}
