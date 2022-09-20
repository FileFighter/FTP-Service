use async_trait::async_trait;
use libunftp::auth::UserDetail;
use libunftp::storage::{Error, Fileinfo, Metadata, Result, StorageBackend};
use std::fmt::Debug;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileFighter {
    // IDEA: maybe uri or smth.
    host: String,
}

#[derive(Debug)]
pub struct InodeMetaData {}

#[async_trait]
impl<User: UserDetail> StorageBackend<User> for FileFighter {
    type Metadata = InodeMetaData;

    fn supported_features(&self) -> u32 {
        todo!()
    }

    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &User,
        path: P,
    ) -> Result<Self::Metadata> {
        todo!()
    }

    async fn list<P>(
        &self,
        user: &User,
        path: P,
    ) -> Result<Vec<Fileinfo<std::path::PathBuf, Self::Metadata>>>
    where
        P: AsRef<Path> + Send + Debug,
        <Self as StorageBackend<User>>::Metadata: Metadata,
    {
        todo!()
    }

    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &User,
        path: P,
        start_pos: u64,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Send + Sync + Unpin>> {
        todo!()
    }

    async fn put<P: AsRef<Path> + Send, R: tokio::io::AsyncRead + Send + Sync + 'static + Unpin>(
        &self,
        user: &User,
        bytes: R,
        path: P,
        start_pos: u64,
    ) -> Result<u64> {
        todo!()
    }

    async fn del<P: AsRef<Path> + Send + Debug>(&self, _user: &User, path: P) -> Result<()> {
        todo!()
    }

    async fn rmd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, path: P) -> Result<()> {
        todo!()
    }

    async fn mkd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, path: P) -> Result<()> {
        todo!()
    }

    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &User,
        from: P,
        to: P,
    ) -> Result<()> {
        todo!()
    }

    async fn cwd<P: AsRef<Path> + Send + Debug>(&self, user: &User, path: P) -> Result<()> {
        todo!()
    }
}

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
