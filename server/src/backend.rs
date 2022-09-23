use crate::metadata::InodeMetaData;
use async_trait::async_trait;
use filefighter_api::ffs_api::{
    endpoints::{create_directory, get_contents_of_folder},
    ApiConfig,
    ApiError::{ReqwestError, ResponseMalformed},
};
use libunftp::storage::{
    Error, ErrorKind, Fileinfo, Metadata, Result, StorageBackend, FEATURE_RESTART,
};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::io::AsyncRead;
use tracing::{debug, info, instrument, warn};
use unftp_auth_filefighter::FileFighterUser;

#[derive(Debug)]
pub struct FileFighter {
    api_config: ApiConfig,
}

impl FileFighter {
    pub fn new() -> Self {
        FileFighter {
            api_config: ApiConfig {
                base_url: "http://localhost:8080/api".to_owned(),
            },
        }
    }
}

#[async_trait]
impl StorageBackend<FileFighterUser> for FileFighter {
    type Metadata = InodeMetaData;

    #[instrument(skip(self), level = "debug")]
    fn supported_features(&self) -> u32 {
        FEATURE_RESTART
    }

    #[instrument(skip(self), level = "debug")]
    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<Self::Metadata> {
        todo!()
    }

    #[instrument(skip(self), level = "debug")]
    async fn list<P>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<Vec<Fileinfo<PathBuf, Self::Metadata>>>
    where
        P: AsRef<Path> + Send + Debug,
        <Self as StorageBackend<FileFighterUser>>::Metadata: Metadata,
    {
        let path = path.as_ref().to_owned();
        let contents = get_contents_of_folder(&self.api_config, &user.token, path)
            .await
            .map_err(|err| match err {
                ReqwestError(err) => {
                    warn!("Cought reqwest error {}", err);
                    Error::new(ErrorKind::LocalError, "Internal Server Error")
                }
                ResponseMalformed(err) => {
                    debug!("Filesystemservice error response: {}", err);
                    Error::new(ErrorKind::PermanentDirectoryNotAvailable, err)
                }
            })?;

        debug!("Found {} inodes", contents.inodes.len());

        Ok(contents
            .inodes
            .iter()
            .map(|inode| Fileinfo {
                path: PathBuf::from(&inode.path),
                metadata: InodeMetaData::new(inode, &contents.owner),
            })
            .collect())
    }

    #[instrument(skip(self), level = "debug")]
    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
        start_pos: u64,
    ) -> Result<Box<dyn AsyncRead + Send + Sync + Unpin>> {
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
        ByteStream: AsyncRead + Send + Sync + 'static + Unpin,
    {
        todo!()
    }

    #[instrument(skip(self), level = "debug")]
    async fn del<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument(skip(self), level = "debug")]
    async fn mkd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        let path = path.as_ref().to_owned();
        let (parent_path, name) = match (path.parent(), path.file_name()) {
            (Some(parent), Some(name)) => Ok((parent, name)),
            (_, _) => Err(Error::new(
                ErrorKind::FileNameNotAllowedError,
                "Path for creating a directoy must contain a parent and child component",
            )),
        }?;

        create_directory(
            &self.api_config,
            &user.token,
            parent_path,
            name.to_str().unwrap(),
        )
        .await
        .map_err(|err| match err {
            ReqwestError(err) => {
                warn!("Cought reqwest error {}", err);
                Error::new(ErrorKind::LocalError, "Internal Server Error")
            }
            ResponseMalformed(err) => {
                debug!("Filesystemservice error response: {}", err);
                Error::new(ErrorKind::PermanentDirectoryNotAvailable, err)
            }
        })?;
        Ok(())
    }

    #[instrument(skip(self), level = "debug")]
    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        from: P,
        to: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument(skip(self), level = "debug")]
    async fn rmd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        todo!()
    }

    #[instrument(skip(self), level = "debug")]
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
