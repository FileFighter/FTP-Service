use chrono::NaiveDateTime;
use filefighter_api::ffs_api::ApiError::{self, ReqwestError, ResponseMalformed};
use libunftp::storage::{
    Error,
    ErrorKind::{self, FileNameNotAllowedError},
    Result,
};
use std::path::{Component, Path, PathBuf};
use tracing::{debug, warn};

pub fn get_parent_and_name(path: &Path) -> Result<(PathBuf, &str)> {
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

pub fn transform_to_ftp_error(error: ApiError) -> Error {
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
pub fn path_contains_rclone_modification_date(path: &Path) -> Option<(NaiveDateTime, PathBuf)> {
    let mut components: Vec<Component> = path.components().collect();

    // needs to be at least / and date<whitespace>
    if components.len() < 2 {
        return None;
    }

    // does exist
    #[allow(clippy::unwrap_used)]
    let mut timestamp_component = components.get(1).unwrap().as_os_str().to_str()?.to_owned();

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

/// Normalizing (without disk access) and validating Path
///
/// The path is validated so that the normalized Path contains no relative components.
pub fn validate_and_normalize_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let normalized_path = normalize_path(path);
    debug!("Normalized: {}", normalized_path.display());

    let path_as_str = normalized_path
        .to_str()
        .ok_or_else(|| Error::new(FileNameNotAllowedError, "Path was not a valid utf8 text"))?;

    if path_as_str.contains("./") || path_as_str.contains("/.") || path_as_str.contains("..") {
        Err(Error::new(
            FileNameNotAllowedError,
            "Path contained relative elements after normalizing",
        ))
    } else {
        Ok(normalized_path)
    }
}

/// Improve the path to try remove and solve .. token.
///
/// Taken from [here](https://github.com/Canop/broot/blob/master/src/path/normalize.rs)
/// This assumes that `a/b/../c` is `a/c` which might be different from
/// what the OS would have chosen when b is a link. This is OK
/// for broot verb arguments but can't be generally used elsewhere
/// (a more general solution would probably query the FS and just
/// resolve b in case of links).
///
/// This function ensures a given path ending with '/' still
/// ends with '/' after normalization.
fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let ends_with_slash = path.as_ref().to_str().map_or(false, |s| s.ends_with('/'));
    let mut normalized = PathBuf::new();
    for component in path.as_ref().components() {
        match &component {
            Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push(component);
                }
            }
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::Normal(_) => {
                normalized.push(component);
            }
        }
    }
    if ends_with_slash {
        normalized.push("");
    }
    normalized
}
