use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn create_file_with_dirs<P: AsRef<Path>>(path: P) -> io::Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    OpenOptions::new().write(true).create(true).open(path)
}

pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    if let Some(parent) = to.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(from, to)
}

pub fn relative_to_absolute<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    fs::canonicalize(path)
}

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

pub fn pathbuf_to_unix_style(path: &PathBuf) -> PathBuf {
    PathBuf::from(path.to_str().unwrap().replace("\\", "/"))
}

pub fn pathbuf_to_win_style(path: &PathBuf) -> PathBuf {
    PathBuf::from(path.to_str().unwrap().replace("/", "\\"))
}

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
