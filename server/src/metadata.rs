use libunftp::storage::{Metadata, Result};
use std::time::SystemTime;

#[derive(Debug)]
pub struct InodeMetaData {}

impl Metadata for InodeMetaData {
    fn len(&self) -> u64 {
        todo!()
    }

    fn is_dir(&self) -> bool {
        todo!()
    }

    fn is_file(&self) -> bool {
        todo!()
    }

    fn is_symlink(&self) -> bool {
        todo!()
    }

    fn modified(&self) -> Result<SystemTime> {
        todo!()
    }

    fn gid(&self) -> u32 {
        todo!()
    }

    fn uid(&self) -> u32 {
        todo!()
    }
}
