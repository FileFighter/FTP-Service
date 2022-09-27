use super::metadata::InodeMetaData;
use crate::auth::user::FileFighterUser;
use async_trait::async_trait;
use filefighter_api::ffs_api::{
    endpoints::{
        create_directory, delete_inode, download_file, get_contents_of_folder, get_inode,
        move_inode, rename_inode, upload_file,
    },
    ApiConfig,
    ApiError::{self, ReqwestError, ResponseMalformed},
};
use libunftp::storage::{
    Error, ErrorKind, Fileinfo, Metadata, Result, StorageBackend, FEATURE_RESTART,
};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};
use tokio::io::AsyncRead;
use tracing::{debug, error, instrument, warn};

#[derive(Debug)]
pub struct FileFighter {
    api_config: ApiConfig,
}

impl FileFighter {
    // IDEA: this should be passed down via a commandline options
    #[must_use]
    pub fn new() -> Self {
        Self {
            api_config: ApiConfig {
                fss_base_url: "http://localhost:8080/api".to_owned(),
                fhs_base_url: "http://localhost:5000/data".to_owned(),
            },
        }
    }
}

impl Default for FileFighter {
    fn default() -> Self {
        Self::new()
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
        let path = path.as_ref().to_owned();
        let inode = get_inode(&self.api_config, &path, &user.token)
            .await
            .map_err(transform_to_ftp_error)?;

        Ok(InodeMetaData::from(&inode, user.id))
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
        let contents = get_contents_of_folder(&self.api_config, &user.token, &path)
            .await
            .map_err(transform_to_ftp_error)?;

        debug!("Found {} inodes", contents.inodes.len());

        Ok(contents
            .inodes
            .iter()
            .map(|inode| Fileinfo {
                path: PathBuf::from(&inode.path),
                metadata: InodeMetaData::from(inode, contents.owner.id),
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
        // IDEA: maybe implement this by skipping the first bytes
        if start_pos != 0 {
            error!("Gets at offset not equal to 0 are not implemented.");
            return Err(Error::new(
                ErrorKind::CommandNotImplemented,
                "Gets at offset not equal to 0 are not implemented.",
            ));
        }

        // TODO: change that everywhere to as_ref() and without &
        let path = path.as_ref().to_owned();

        download_file(&self.api_config, &user.token, &path)
            .await
            .map_err(transform_to_ftp_error)
    }

    #[instrument(skip(self, bytes))]
    async fn put<FilePath, ByteStream>(
        &self,
        user: &FileFighterUser,
        bytes: ByteStream,
        path: FilePath,
        start_pos: u64,
    ) -> Result<u64>
    where
        FilePath: AsRef<Path> + Send + Debug,
        ByteStream: AsyncRead + Send + Sync + 'static + Unpin,
    {
        // TODO: remove this by implementing
        if start_pos != 0 {
            error!("Puts at offset not equal to 0 are not implemented.");
            return Err(Error::new(
                ErrorKind::CommandNotImplemented,
                "Puts at offset not equal to 0 are not implemented.",
            ));
        }

        let path = path.as_ref().to_owned();
        let (parent_path, name) = get_parent_and_name(&path)?;

        upload_file(&self.api_config, &user.token, &parent_path, &name, bytes)
            .await
            .map_err(transform_to_ftp_error)?;

        let inode = get_inode(&self.api_config, &path, &user.token)
            .await
            .map_err(transform_to_ftp_error)?;

        Ok(inode.size)
    }

    #[instrument(skip(self), level = "debug")]
    async fn del<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        // Should this check if the inode to delete is really a file?
        let path = path.as_ref().to_owned();
        delete_inode(&self.api_config, &user.token, &path)
            .await
            .map_err(transform_to_ftp_error)?;
        Ok(())
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
            .map_err(transform_to_ftp_error)?;
        Ok(())
    }

    /// Used to rename and move inodes.
    /// TODO: fix this by implementing a custom endpoint in the fss
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
                .map_err(transform_to_ftp_error)?
                .path;
            from_path = PathBuf::from(new_path);
        };

        if from_parent != to_parent {
            move_inode(&self.api_config, &user.token, &from_path, &to_parent)
                .await
                .map_err(transform_to_ftp_error)?;
        };

        Ok(())
    }

    // IDEA: check if inode at path is a directory
    #[instrument(skip(self), level = "debug")]
    async fn rmd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        let path = path.as_ref().to_owned();
        delete_inode(&self.api_config, &user.token, &path)
            .await
            .map_err(transform_to_ftp_error)?;
        Ok(())
    }

    // TODO: normalize path without interacting with the fs (. and .. // )
    // TODO: check that path is a folder
    #[instrument(skip(self), level = "debug")]
    async fn cwd<P: AsRef<Path> + Send + Debug>(
        &self,
        _user: &FileFighterUser,
        _path: P,
    ) -> Result<()> {
        Ok(())
    }
}

fn get_parent_and_name(path: &Path) -> Result<(PathBuf, &str)> {
    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => Ok((parent.to_path_buf(), name.to_str().unwrap())),
        (_, _) => Err(Error::new(
            ErrorKind::FileNameNotAllowedError,
            "Path for creating a directory must contain a parent and child component",
        )),
    }
}

fn transform_to_ftp_error(error: ApiError) -> Error {
    match error {
        ReqwestError(err) => {
            warn!("Cought reqwest error {}", err);
            Error::new(ErrorKind::LocalError, "Internal Server Error")
        }
        ResponseMalformed(err) => {
            warn!("Filesystemservice error response: {}", err);
            Error::new(ErrorKind::PermanentDirectoryNotAvailable, err)
        }
    }
}
