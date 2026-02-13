use core::convert::AsRef;
use std::fs::{self, File, OpenOptions};
use std::io::{ Error, ErrorKind, Result, Write };
use std::path::{Path, PathBuf, StripPrefixError};

use log::debug;

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

/// Write content to a file
pub fn write_diff_file_to_trans<P: AsRef<Path>>(to: P, content: &str) -> Result<()> {
    let path_rel_to_root = get_path_rel_to_root(&PathBuf::from(to.as_ref()));
    let mut to = get_trans_dir().join(path_rel_to_root);
    if let Some(fname) = to.file_name().and_then(|s| s.to_str()) {
        to.set_file_name(format!("{}.diff", fname));
    }
    let mut file = create_file_with_dirs(to)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Copy files in .trans folder to root directory
pub fn cover() -> Result<u64> {
    let from = get_trans_dir();
    let to_root = get_root_dir().unwrap();
    // recursively copy everything under .trans into root, skipping the records file
    copy_dir_recursive(&from, &to_root, &from)
}

/// Recursively walk a source directory and copy all files to the destination root,
/// preserving the tree structure. `base` is the top of the recursion and is used
/// to compute the relative path for each entry. The function returns the number of
/// files copied. Existing files are always overwritten.
fn copy_dir_recursive(src: &Path, dest_root: &Path, base: &Path) -> Result<u64> {
    let mut count = 0;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        // skip the records file at the top level
        if path.file_name().and_then(|n| n.to_str()) == Some("records.toml") {
            continue;
        }

        if path.is_dir() {
            // recurse into subdirectory
            count += copy_dir_recursive(&path, dest_root, base)?;
        } else if path.is_file() {
            let rel = path.strip_prefix(base).unwrap();
            let dest = dest_root.join(rel);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            debug!("cover:\nfrom: {:?},\nto: {:?}", path, dest);
            if let Err(e) = copy_file(&path, &dest, true) {
                debug!("cover error: {:?}", e);
            }
            count += 1;
        }
    }
    Ok(count)
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
) -> std::result::Result<PathBuf, StripPrefixError> {
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
