use crate::metadata::InodeMetaData;
use async_trait::async_trait;
use filefighter_api::ffs_api::{
    endpoints::{create_directory, get_contents_of_folder, move_inode, rename_inode},
    ApiConfig,
    ApiError::{ReqwestError, ResponseMalformed},
};
use libunftp::storage::{
    Error, ErrorKind, Fileinfo, Metadata, Result, StorageBackend, FEATURE_RESTART,
};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
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
        let (parent_path, name) = get_parent_and_name(&path)?;

        create_directory(&self.api_config, &user.token, parent_path.as_path(), name)
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
        let mut from_path = from.as_ref().to_owned();
        let to_path = to.as_ref().to_owned();

        let (from_parent, from_name) = get_parent_and_name(&from_path)?;
        let (to_parent, to_name) = get_parent_and_name(&to_path)?;

        if from_name != to_name {
            let new_path = rename_inode(&self.api_config, &user.token, &from_path, to_name)
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
                })?
                .path;
            from_path = PathBuf::from(new_path)
        };

        if from_parent != to_parent {
            move_inode(&self.api_config, &user.token, &from_path, &to_parent)
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
        };

        Ok(())
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

fn get_parent_and_name<'a>(path: &'a PathBuf) -> Result<(PathBuf, &'a str)> {
    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => Ok((parent.to_path_buf(), name.to_str().unwrap())),
        (_, _) => Err(Error::new(
            ErrorKind::FileNameNotAllowedError,
            "Path for creating a directory must contain a parent and child component",
        )),
    }
}
