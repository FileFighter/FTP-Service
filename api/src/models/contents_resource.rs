use super::{inode_resource::InodeResource, user_resource::UserResource};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentsResource {
    #[serde(rename = "inodes")]
    inodes: Vec<InodeResource>,
    #[serde(rename = "owner")]
    owner: UserResource,
}
