use super::{
    metadata::InodeMetaData,
    utils::{
        get_parent_and_name, path_contains_rclone_modification_date, transform_to_ftp_error,
        validate_and_normalize_path,
    },
};
use crate::auth::user::FileFighterUser;
use async_trait::async_trait;
use filefighter_api::ffs_api::{
    endpoints::{
        create_directory, delete_inode, download_file, get_contents_of_folder, get_inode,
        move_inode, rename_inode, set_last_modified_of_inode, upload_file,
    },
    ApiConfig,
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
    pub api_config: ApiConfig,
}

#[async_trait]
impl StorageBackend<FileFighterUser> for FileFighter {
    type Metadata = InodeMetaData;

    #[instrument(skip(self), level = "debug")]
    fn supported_features(&self) -> u32 {
        FEATURE_RESTART
    }

    /// Endpoint to request Metadata for a inode
    ///
    /// # Rclone
    /// In some cases the path consists of /<date>/ /acutal_path.
    /// This means that rclone wants to update the modification date of that inode at the path
    #[instrument(skip(self), level = "debug")]
    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<Self::Metadata> {
        let path = path.as_ref();

        let inode = match path_contains_rclone_modification_date(path) {
            // rclone wants to update time
            Some(tuple) => {
                set_last_modified_of_inode(
                    &self.api_config,
                    &user.token,
                    &tuple.1,
                    tuple.0.timestamp(),
                )
                .await
            }
            // regular metadata request
            None => {
                get_inode(
                    &self.api_config,
                    &validate_and_normalize_path(path)?,
                    &user.token,
                )
                .await
            }
        }
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
        let path = validate_and_normalize_path(path)?;
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

        let path = validate_and_normalize_path(path)?;

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

        let path = validate_and_normalize_path(path)?;
        let (parent_path, name) = get_parent_and_name(&path)?;

        upload_file(&self.api_config, &user.token, &parent_path, name, bytes)
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
        let path = validate_and_normalize_path(path)?;
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
        let path = validate_and_normalize_path(path)?;
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
        let mut from_path = validate_and_normalize_path(from)?;
        let to_path = validate_and_normalize_path(to)?;

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
        let path = validate_and_normalize_path(path)?;
        delete_inode(&self.api_config, &user.token, &path)
            .await
            .map_err(transform_to_ftp_error)?;
        Ok(())
    }

    // TODO: check that path is a folder
    #[instrument(skip(self), level = "debug")]
    async fn cwd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &FileFighterUser,
        path: P,
    ) -> Result<()> {
        let path = validate_and_normalize_path(path)?;
        let inode = get_inode(&self.api_config, &path, &user.token)
            .await
            .map_err(transform_to_ftp_error)?;

        // transform to metadata so we can check if its a directory
        let inode_metadata = InodeMetaData::from(&inode, user.id);
        if inode_metadata.is_dir() {
            Ok(())
        } else {
            // IDEA: should we log something here?
            Err(Error::new(
                ErrorKind::PermanentDirectoryNotAvailable,
                "Path didn't point to a directory.",
            ))
        }
    }
}
