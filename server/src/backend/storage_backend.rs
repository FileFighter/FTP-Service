use super::{metadata::InodeMetaData, utils::validate_and_normalize_path};
use crate::auth::user::FileFighterUser;
use async_trait::async_trait;
use chrono::NaiveDateTime;
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
    path::{Component, Path, PathBuf},
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

        todo!()
        // // rclone wants to update time
        // if path_contains_rclone_modification_date(path)? {
        // } else {
        //     // regular metadata request
        //     let path = validate_and_normalize_path(path)?;
        //     let inode = get_inode(&self.api_config, &path, &user.token)
        //         .await
        //         .map_err(transform_to_ftp_error)?;

        //     Ok(InodeMetaData::from(&inode, user.id))
        // }
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

fn get_parent_and_name(path: &Path) -> Result<(PathBuf, &str)> {
    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => Ok((
            parent.to_path_buf(),
            name.to_str()
                .ok_or_else(|| Error::new(ErrorKind::LocalError, "Filename was not valid utf-8"))?,
        )),
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

// IDEA: check if rclone does try to update the root folder
fn path_contains_rclone_modification_date(path: &Path) -> Option<(NaiveDateTime, PathBuf)> {
    let mut components: Vec<Component> = path.components().collect();

    // needs to be at least / and date<whitespace>
    if components.len() < 2 {
        return None;
    }

    // does exist
    let mut timestamp_component = components[1].as_os_str().to_str()?.to_owned();

    #[allow(clippy::unwrap_used)]
    // if root path does not end with whitespace
    if timestamp_component.is_empty() || timestamp_component.pop().unwrap() != ' ' {
        return None;
    }

    // the rest of the root folder needs to be in this format
    // yyyymmddhhmmss
    let parsed_time = NaiveDateTime::parse_from_str(&timestamp_component, "%Y%m%d%H%M%S").ok()?;

    // remove the timestamp component
    components.remove(1);

    Some((parsed_time, components.iter().collect()))
}

#[cfg(test)]
mod tests {
    use super::path_contains_rclone_modification_date;
    use chrono::NaiveDateTime;
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn timestamp_parsing_works() {
        let result = NaiveDateTime::parse_from_str("20221003093709", "%Y%m%d%H%M%S").unwrap();
        let resulting_string = result.to_string();
        assert_eq!("2022-10-03 09:37:09", resulting_string)
    }

    #[test]
    fn path_contains_rclone_modification_date_works() {
        let path = PathBuf::from_str("/20221003093709 /Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);

        match option {
            Some(result) => {
                assert_eq!(
                    NaiveDateTime::parse_from_str("20221003093709", "%Y%m%d%H%M%S").unwrap(),
                    result.0
                );
                assert_eq!(PathBuf::from_str("/Home/School").unwrap(), result.1);
            }
            None => panic!("Expected some value here."),
        }
    }

    #[test]
    fn path_contains_rclone_modification_date_fails_without_whitespace() {
        let path = PathBuf::from_str("/20221003093709/Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);
        assert!(option.is_none())
    }

    #[test]
    fn path_contains_rclone_modification_date_fails_with_wrong_timestamp_format() {
        let path = PathBuf::from_str("/202210030937 /Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);
        assert!(option.is_none())
    }

    #[test]
    fn path_contains_rclone_modification_date_fails_with_wrong_timestamp() {
        let path = PathBuf::from_str("/20221003093790 /Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);
        assert!(option.is_none())
    }
}
