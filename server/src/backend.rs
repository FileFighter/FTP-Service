use crate::metadata::InodeMetaData;
use async_trait::async_trait;
use libunftp::storage::{Fileinfo, Metadata, Result, StorageBackend, FEATURE_RESTART};
use std::{fmt::Debug, path::Path};
use tracing::instrument;
use unftp_auth_filefighter::FileFighterUser;

#[derive(Debug)]
pub struct FileFighter;

impl FileFighter {
    pub fn new() -> Self {
        FileFighter {}
    }
}

#[async_trait]
impl StorageBackend<FileFighterUser> for FileFighter {
    type Metadata = InodeMetaData;

    #[instrument]
    fn supported_features(&self) -> u32 {
        FEATURE_RESTART
    }

    #[instrument]
    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<Self::Metadata> {
        todo!()
    }

    #[instrument]
    async fn list<P>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<Vec<Fileinfo<std::path::PathBuf, Self::Metadata>>>
    where
        P: AsRef<Path> + Send + Debug,
        <Self as StorageBackend<FileFighterUser>>::Metadata: Metadata,
    {
        todo!()
    }

    #[instrument]
    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
        start_pos: u64,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Send + Sync + Unpin>> {
        todo!()
    }

    #[instrument(skip(bytes, path))]
    async fn put<FilePath, ByteStream>(
        &self,
        user: &FileFighterUser,
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
    async fn del<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn rmd<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn mkd<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        from: P,
        to: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument]
    async fn cwd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        // TODO: normalize path without interacting with the fs (. and .. // )
        // TODO: check that path is a folder
        Ok(())
    }
}
