use crate::metadata::InodeMetaData;
use async_trait::async_trait;
use libunftp::auth::UserDetail;
use libunftp::storage::{Fileinfo, Metadata, Result, StorageBackend};
use std::{fmt::Debug, path::Path};
use tracing::instrument;

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

    #[instrument]
    fn supported_features(&self) -> u32 {
        todo!()
    }

    #[instrument]
    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &User,
        path: P,
    ) -> Result<Self::Metadata> {
        todo!()
    }

    #[instrument]
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

    #[instrument]
    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &User,
        path: P,
        start_pos: u64,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Send + Sync + Unpin>> {
        todo!()
    }

    #[instrument(skip(bytes, path))]
    async fn put<FilePath, ByteStream>(
        &self,
        user: &User,
        bytes: ByteStream,
        path: FilePath,
        start_pos: u64,
    ) -> Result<u64>
    where
        FilePath: AsRef<Path> + Send,
        ByteStream: tokio::io::AsyncRead + Send + Sync + 'static + Unpin,
    {
        todo!()
    }

    #[instrument]
    async fn del<P: AsRef<Path> + Send + Debug>(&self, _user: &User, path: P) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn rmd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, path: P) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn mkd<P: AsRef<Path> + Send + Debug>(&self, _user: &User, path: P) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &User,
        from: P,
        to: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn cwd<P: AsRef<Path> + Send + Debug>(&self, user: &User, path: P) -> Result<()> {
        todo!()
    }
}
