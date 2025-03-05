use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::git::get_root_dir;

/// # File handling functions

/// Create a file with all the necessary directories
pub fn create_file_with_dirs<P: AsRef<Path>>(path: P) -> io::Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    OpenOptions::new().write(true).create(true).open(path)
}

/// Copy a file to a new location with all the necessary directories
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    if let Some(parent) = to.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(from, to)
}

/// # Path handling functions

pub fn get_trans_dir() -> PathBuf {
    return get_root_dir().unwrap().join(".trans");
}

pub fn get_records_toml() -> PathBuf {
    return get_trans_dir().join("records.toml");
}

/// Convert a relative path to an absolute path
pub fn relative_to_absolute<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    fs::canonicalize(path)
}

/// Convert an absolute path to a relative path
pub fn absolute_to_relative<P: AsRef<Path>, Q: AsRef<Path>>(
    base: P,
    path: Q,
) -> io::Result<PathBuf> {
    let base = base.as_ref();
    let path = path.as_ref();
    path.strip_prefix(base)
        .map(PathBuf::from)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Convert a path to a unix style path
pub fn pathbuf_to_unix_style(path: &PathBuf) -> PathBuf {
    PathBuf::from(path.to_str().unwrap().replace("\\", "/"))
}

/// Convert a path to a win style path
pub fn pathbuf_to_win_style(path: &PathBuf) -> PathBuf {
    PathBuf::from(path.to_str().unwrap().replace("/", "\\"))
}

pub fn get_path_rel_to_root(path: &PathBuf) -> PathBuf {
    let root_dir: PathBuf = get_root_dir().unwrap();
    let path = pathbuf_to_unix_style(path);
    return pathbuf_to_unix_style(
        &absolute_to_relative::<PathBuf, PathBuf>(
            root_dir,
            relative_to_absolute(path.clone()).unwrap(),
        )
        .unwrap(),
    );
}

/// Strip the trailing newline from a string
pub fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn main() -> io::Result<()> {
    let file_path = "some/non/existent/directory/file.txt";
    let mut file = create_file_with_dirs(file_path)?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
