use filefighter_api::ffs_api::models::inode_resource::InodeResource;
use libunftp::storage::{Metadata, Result};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct InodeMetaData {
    len: u64,
    is_file: bool,
    modified: SystemTime,
    gid: u32,
    uid: u32,
}

impl Metadata for InodeMetaData {
    fn len(&self) -> u64 {
        self.len
    }

    fn is_dir(&self) -> bool {
        !self.is_file
    }

    fn is_file(&self) -> bool {
        self.is_file
    }

    fn is_symlink(&self) -> bool {
        // we dont have symlinks (maybe with sharing?)
        false
    }

    fn modified(&self) -> Result<SystemTime> {
        Ok(self.modified)
    }

    fn gid(&self) -> u32 {
        self.gid
    }

    fn uid(&self) -> u32 {
        self.uid
    }
}

impl InodeMetaData {
    pub fn from(inode: &InodeResource, owner_id: u32) -> Self {
        Self {
            len: inode.size,
            is_file: inode.mime_type.is_some(),
            // TODO: does this work?
            modified: UNIX_EPOCH + Duration::from_secs(inode.last_updated),
            gid: owner_id,
            uid: owner_id,
        }
    }
}
