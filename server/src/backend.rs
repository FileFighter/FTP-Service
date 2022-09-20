use crate::metadata::InodeMetaData;
use async_trait::async_trait;
use libunftp::auth::UserDetail;
use libunftp::storage::{Fileinfo, Metadata, Result, StorageBackend};
use std::{fmt::Debug, path::Path};

#[derive(Debug)]
pub struct FileFighter;

impl FileFighter {
    pub fn new() -> Self {
        FileFighter {}
    }
}

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
