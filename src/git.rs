use std::{error::Error, fs, path::Path, path::PathBuf, process::Command};

/// Get the root directory of the git repository
pub fn get_root_dir() -> Option<PathBuf> {
    let root_dir = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("failed to execute: git rev-parse --show-toplevel");

    if root_dir.status.code().unwrap() != 0 {
        eprintln!("not a git repository");
        return None;
    }

    let root_dir = String::from_utf8_lossy(&root_dir.stdout).trim().to_string();
    let absolute_path = fs::canonicalize(root_dir)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if cfg!(windows) {
        return Some(PathBuf::from(absolute_path.replace("/", "\\")));
    }
    Some(PathBuf::from(absolute_path))
}

/// Get current directory path prefix
#[allow(dead_code)]
pub fn get_prefix() -> Option<PathBuf> {
    let prefix = Command::new("git")
        .args(["rev-parse", "--show-prefix"])
        .output()
        .expect("failed to execute: git rev-parse --show-prefix");

    if prefix.status.code().unwrap() != 0 {
        eprintln!("not a git repository");
        return None;
    }

    let prefix = String::from_utf8_lossy(&prefix.stdout).trim().to_string();

    if cfg!(windows) {
        return Some(PathBuf::from(prefix.replace("/", "\\")));
    }
    Some(PathBuf::from(prefix))
}

/// Get the revision from a tag
pub fn get_tag_rev(tag: &String) -> Option<String> {
    let revision = Command::new("git")
        .args(["rev-parse", tag])
        .output()
        .unwrap_or_else(|_| panic!("failed to execute: git rev-parse {}", tag));

    if revision.status.code().unwrap() != 0 {
        eprintln!("not a git revision");
        return None;
    }

    let revision = String::from_utf8_lossy(&revision.stdout).trim().to_string();
    Some(revision)
}

/// Get the current revision of a file
pub fn get_file_rev(path: &Path) -> String {
    let file_revision = Command::new("git")
        .args(["log", "-n", "1", "--pretty=format:%H", "--"])
        .arg(path)
        .output()
        .expect("failed to execute: git log -n 1 --pretty=format:%H -- <path>");
    String::from_utf8_lossy(&file_revision.stdout)
        .trim()
        .to_string()
}

/// Get diff between two revisions of a file
pub fn get_diff(path: &Path, old_rev: &str, new_rev: &str) -> String {
    let diff = Command::new("git")
        .args(["diff", old_rev, new_rev, path.to_str().unwrap()])
        .output()
        .expect("failed to execute: git diff {old_rev} {new_rev} {path}");
    String::from_utf8_lossy(&diff.stdout).to_string()
}

/// Show logs in the .trans folder
pub fn get_log(path: &Path) -> String {
    let log = Command::new("git")
        .args(["log", "--", path.to_str().unwrap()])
        .output()
        .expect("failed to execute: git log -- {path}");
    String::from_utf8_lossy(&log.stdout).to_string()
}

/// Reset the root folder to the latest revision
pub fn reset() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .args([
            "restore",
            "--source=HEAD",
            "--staged",
            "--worktree",
            ".",
            ":(exclude).trans/",
        ])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err("failed to execute: git reset --hard HEAD -- . :!.trans/"
            .to_string()
            .into())
    }
}
