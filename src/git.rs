use std::{fs, path::PathBuf, process::Command};

enum GitCmd {
    RepoDir,
    HeadHash,
    FileRevision,
}
pub fn git() {}

/// Get the root directory of the git repository
pub fn get_root_dir() -> Option<PathBuf> {
    let root_dir = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("failed to execute: git rev-parse --show-toplevel");

    if root_dir.status.code().unwrap() != 0 {
        eprintln!("not a git repository");
        return None;
    } else {
        let root_dir = String::from_utf8_lossy(&root_dir.stdout).trim().to_string();
        let absolute_path = fs::canonicalize(root_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if cfg!(windows) {
            return Some(PathBuf::from(absolute_path.replace("/", "\\")));
        }
        return Some(PathBuf::from(absolute_path));
    }
}

/// Get the hash of the current commit
pub fn get_head_hash() -> String {
    let head_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute: git rev-parse HEAD");
    let head_hash = String::from_utf8_lossy(&head_hash.stdout)
        .trim()
        .to_string();
    return head_hash;
}

pub fn get_file_revision(path: &PathBuf) -> String {
    let file_revision = Command::new("git")
        .args(["log", "-n", "1", "--pretty=format:%H", "--"])
        .arg(path)
        .output()
        .expect("failed to execute: git log -n 1 --pretty=format:%H -- <path>");
    let file_revision = String::from_utf8_lossy(&file_revision.stdout)
        .trim()
        .to_string();
    return file_revision;
}
