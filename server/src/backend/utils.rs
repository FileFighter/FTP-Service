use libunftp::storage::{Error, ErrorKind::FileNameNotAllowedError, Result};
use std::path::{Component, Path, PathBuf};
use tracing::debug;

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
