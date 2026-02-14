use chrono::Local;
use clap::{Subcommand, ValueEnum};
use core::{option::Option::None, result::Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fs, io::{Error, ErrorKind}, path::PathBuf, str::FromStr, fmt::Display
};
use toml::value::Datetime;

use crate::{git::*, utils::*};

/// Records file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Records {
    pub meta: Meta,
    pub files: Vec<TrackedFile>,
}

/// Records file meta information
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    /// Project name
    pub project_name: String,
    /// Language code, like zh-CN, en-US, etc.
    pub lang: String,
    /// track repo revision, hash or tag
    pub track_rev: String,
    /// Local datetime, rfc3339 format
    pub datetime: Datetime,
}

/// File status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Subcommand, ValueEnum)]
pub enum Status {
    /// File is to be translated
    Todo,
    /// File is to be reviewed
    ToReview,
    /// File is done
    Done,
    /// File is synced
    Synced,
    /// File is unsynced
    Unsynced,
    /// File is locked
    Lock,
    /// File is unlocked
    Unlock,
}

impl Status {
    /// Iterate through every possible `Status` variant.
    pub fn iter() -> impl Iterator<Item = Status> {
        [
            Status::Todo,
            Status::ToReview,
            Status::Done,
            Status::Synced,
            Status::Unsynced,
        ]
        .iter()
        .cloned()
    }
}
impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "Todo" => Ok(Status::Todo),
            "ToReview" => Ok(Status::ToReview),
            "Done" => Ok(Status::Done),
            "Synced" => Ok(Status::Synced),
            "Unsynced" => Ok(Status::Unsynced),
            _ => Err(()),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackedFile {
    pub path: PathBuf,
    pub track_rev: String,
    pub status: Status,
    pub locked: Option<bool>,
}

impl Records {
    /// initial records.toml
    pub fn init(lang: &String, tag: &String) -> Result<Records, Error> {
        let root_dir: PathBuf = get_root_dir().unwrap();
        let project_name = root_dir.file_name().unwrap().to_str().unwrap().to_string();
        if let Some(rev) = get_tag_revision(tag) {
            return Ok(Records {
                meta: Meta {
                    project_name: project_name,
                    lang: lang.clone(),
                    track_rev: if tag == "HEAD" { rev } else { tag.clone() },
                    datetime: Datetime::from_str(&Local::now().to_rfc3339()).unwrap(),
                },
                files: Vec::new(),
            });
        } else {
            let err = Error::new(
                ErrorKind::InvalidInput,
                format!("{tag} is not a valid revision"),
            );
            return Err(err);
        }
    }
    /// Add file to records
    pub fn add(&mut self, path: &PathBuf, lock: bool) -> Result<TrackedFile, Error> {
        let path = unify(&path);
        if self.contains(&path) {
            let err = Error::new(ErrorKind::AlreadyExists, "record already exists");
            return Err(err);
        }

        let path_rel_to_root = get_path_rel_to_root(&path);
        let file = TrackedFile {
            path: path_rel_to_root,
            track_rev: get_file_revision(&path),
            status: Status::Todo,
            locked: if lock { Some(true) } else { None },
        };
        self.files.push(file.clone());
        return Ok(file);
    }

    /// Remove file from records
    pub fn remove(&mut self, path: &PathBuf) -> Result<TrackedFile, Error> {
        let path = unify(&path);
        let path_rel_to_root = get_path_rel_to_root(&path);

        if let Some(pos) = self
            .files
            .iter()
            .position(|file| file.path == path_rel_to_root)
        {
            let removed_record = self.files.remove(pos);
            Ok(removed_record)
        } else {
            let err = Error::new(ErrorKind::NotFound, "record not found");
            Err(err)
        }
    }

    /// Update file in records
    pub fn update<F>(&mut self, path: &PathBuf, modify_fn: F) -> Result<TrackedFile, Error>
    where
        F: FnOnce(&mut TrackedFile),
    {
        let path = unify(&path);
        let path_rel_to_root = get_path_rel_to_root(&path);

        if let Some(file) = self
            .files
            .iter_mut()
            .find(|file| file.path == path_rel_to_root)
        {
            modify_fn(file);
            let file_result = file.clone();
            // End mutable borrow before serializing self
            self.save()?;
            Ok(file_result)
        } else {
            let err = Error::new(ErrorKind::NotFound, "record not found");
            Err(err)
        }
    }

    /// Save records to records.toml
    pub fn save(&self) -> std::io::Result<()> {
        let toml = toml::to_string(self).unwrap();
        fs::write(get_records_toml(), toml)
    }

    /// Show files with specific status
    pub fn show(&self, status: Status) {
        let files = self.files.iter().filter(|file| file.status == status);
        if files.clone().count() == 0 {
            println!("\nNo files with status {:?}", status);
            return;
        }
        for file in files {
            println!("{:?}: {}", file.status, file.path.display());
        }
    }

    /// Mark file status in records
    pub fn mark(&mut self, status: Status, path: &PathBuf) -> Result<TrackedFile, Error> {
        match status {
            Status::Lock => {
                let lock = |file: &mut TrackedFile| file.locked = Some(true);
                return self.update(path, lock);
            }
            Status::Unlock => {
                let unlock = |file: &mut TrackedFile| {
                    if file.locked.is_some() {
                        file.locked = None;
                    }
                };
                return self.update(path, unlock);
            }
            _ => {
                let mark = |file: &mut TrackedFile| file.status = status;
                return self.update(path, mark);
            }
        }
    }

    /// Get file in records
    pub fn get(&mut self, path: &PathBuf) -> Result<TrackedFile, Error> {
        let noop = |_: &mut TrackedFile| ();
        self.update(path, noop)
    }

    /// Sync file revision in records
    pub fn sync(&mut self, path: &PathBuf) -> Result<TrackedFile, Error> {
        let sync = |file: &mut TrackedFile| file.track_rev = get_file_revision(&path);
        self.update(path, sync)
    }

    /// Check if records contains the file
    pub fn contains(&self, path: &PathBuf) -> bool {
        let path = unify(&path);
        let path_rel_to_root = get_path_rel_to_root(&path);
        self.files.iter().any(|file| file.path == path_rel_to_root)
    }
}
