use std::fs::{self, File, OpenOptions};
use std::io::{ Error, ErrorKind, Result};
use std::path::{Path, PathBuf, StripPrefixError};
use std::result;

use crate::git::get_root_dir;

/// # File handling functions

/// Create a file with all the necessary directories
pub fn create_file_with_dirs<P: AsRef<Path>>(path: P) -> Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    if path.as_ref().exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, "file already exists"));
    }
    OpenOptions::new().write(true).create(true).open(path)
}

/// Copy a file to a new location with all the necessary directories
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q, overwrite: bool) -> Result<u64> {
    if !overwrite && to.as_ref().exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, "file already exists"));
    }

    if let Some(parent) = to.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(from, to)
}

/// Copy a file to the .trans directory
pub fn copy_file_to_trans<P: AsRef<Path>>(from: P) -> Result<u64> {
    let path_rel_to_root = get_path_rel_to_root(&PathBuf::from(from.as_ref()));
    let to = get_trans_dir().join(path_rel_to_root);
    println!("from: {:?}, to: {:?}", from.as_ref(), to);
    copy_file(from, to, false)
}

/// # Path handling functions
/// Get the .trans directory
pub fn get_trans_dir() -> PathBuf {
    return get_root_dir().unwrap().join(".trans");
}

/// Get the records.toml file path
pub fn get_records_toml() -> PathBuf {
    return get_trans_dir().join("records.toml");
}

/// Convert an absolute path to a relative path
pub fn absolute_to_relative<P: AsRef<Path>, Q: AsRef<Path>>(
    base: P,
    path: Q,
) -> result::Result<PathBuf, StripPrefixError> {
    let base = base.as_ref();
    let path = path.as_ref();
    path.strip_prefix(base)
        .map(PathBuf::from)
}

/// Convert a path to a unix style path
pub fn unify(path: &PathBuf) -> PathBuf {
    PathBuf::from(path.to_str().unwrap().replace("\\", "/"))
}

/// Get the relative path of a file to the root directory
pub fn get_path_rel_to_root(path: &PathBuf) -> PathBuf {
    let root_dir: PathBuf = get_root_dir().unwrap();
    let path = unify(&path);
    return unify(
        &absolute_to_relative::<PathBuf, PathBuf>(
            root_dir,
            fs::canonicalize(&path).unwrap(),
        )
        .unwrap(),
    );
}
