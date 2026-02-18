use chrono::Local;
use clap::{Subcommand, ValueEnum};
use colored::*;
use core::{option::Option::None, result::Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
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
pub enum Progress {
    /// File is to be translated
    Todo,
    /// File is to be reviewed
    Review,
    /// File is done
    Done,
}

impl Progress {
    /// Iterate through every possible `Status` variant.
    pub fn iter() -> impl Iterator<Item = Progress> {
        [Progress::Todo, Progress::Review, Progress::Done]
            .iter()
            .cloned()
    }
}

impl FromStr for Progress {
    type Err = ();

    fn from_str(input: &str) -> Result<Progress, Self::Err> {
        match input {
            "Todo" => Ok(Progress::Todo),
            "ToReview" => Ok(Progress::Review),
            "Done" => Ok(Progress::Done),
            _ => Err(()),
        }
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackedFile {
    pub path: PathBuf,
    pub track_rev: String,
    pub progress: Progress,
    pub synced: bool,
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
            progress: Progress::Todo,
            synced: true,
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
            self.save()?;
            Ok(file_result)
        } else {
            let err = Error::new(ErrorKind::NotFound, "record not found");
            Err(err)
        }
    }

    /// Get file in records
    pub fn get(&mut self, path: &PathBuf) -> Result<TrackedFile, Error> {
        let noop = |_: &mut TrackedFile| ();
        self.update(path, noop)
    }

    /// Save records to records.toml
    pub fn save(&self) -> std::io::Result<()> {
        let toml = toml::to_string(self).unwrap();
        fs::write(get_records_toml(), toml)
    }

    /// Show all files in records
    pub fn show_all(&self) {
        if self.files.len() == 0 {
            println!("No files in records.");
            return;
        }
        println!("{}, {}, {} | {} | {}", "T: Todo".red(), "R: Review".yellow(), "D: Done".green(), "S: Synced".bright_blue(), "L: Locked".bright_green());
        for file in self.files.iter() {
            let prog = match file.progress {
                Progress::Todo => "T".red(),
                Progress::Review => "R".yellow(),
                Progress::Done => "D".green(),
            };
            let synced = if file.synced == true { "S".bright_blue() } else { "-".white() };
            let lock = if file.locked == Some(true) { "L".bright_green() } else { "-".white() };
            println!("{prog}{synced}{lock}\t{}", file.path.display());
        }
    }

    /// Show files with specific status
    pub fn show_progress(&self, prog: Progress) {
        let files = self.files.iter().filter(|file| file.progress == prog);
        if files.clone().count() == 0 {
            println!("\nNo files are in the {:?} status.", prog);
            return;
        }
        for file in files {
            println!("{}\t{}", file.progress, file.path.display());
        }
    }

    pub fn show_synced(&self, synced: bool) {
        let files = self.files.iter().filter(|file| file.synced == synced);
        if files.clone().count() == 0 {
            println!("\nNo files are {}.", if synced { "Synced" } else { "Unsynced" });
            return;
        }
        for file in files {
            println!(
                "{}\t{}",
                if file.synced { "Synced".bright_blue() } else { "Unsynced".blue() },
                file.path.display()
            );
        }
    }

    pub fn show_locked(&self, locked: bool) {
        let files = self.files.iter().filter(|file| {
            if locked {
                file.locked == Some(true)
            } else {
                file.locked.is_none()
            }
        });
        if files.clone().count() == 0 {
            println!("\nNo files are {}.", if locked { "Locked" } else { "Unlocked" });
            return;
        }
        for file in files {
            println!(
                "{}\t{}",
                if locked == true { "Locked".bright_green() } else { "Unlocked".green() },
                file.path.display()
            );
        }
    }

    /// Mark file status in records
    pub fn mark_progress(&mut self, prog: Progress, path: &PathBuf) -> Result<TrackedFile, Error> {
        let mark_prog = |file: &mut TrackedFile| file.progress = prog;
        return self.update(path, mark_prog);
    }

    /// Sync file revision in records
    pub fn set_synced(&mut self, path: &PathBuf) -> Result<TrackedFile, Error> {
        let sync = |file: &mut TrackedFile| {
            file.track_rev = get_file_revision(&path);
            file.synced = true;
        };
        self.update(path, sync)
    }

    /// Lock file in records
    pub fn set_lock(&mut self, locked: bool, path: &PathBuf) -> Result<TrackedFile, Error> {
        let lock = |file: &mut TrackedFile| {
            if locked {
                file.locked = Some(true);
            } else {
                file.locked = None;
            }
        };
        return self.update(path, lock);
    }

    /// Check if records contains the file
    pub fn contains(&self, path: &PathBuf) -> bool {
        let path = unify(&path);
        let path_rel_to_root = get_path_rel_to_root(&path);
        self.files.iter().any(|file| file.path == path_rel_to_root)
    }
}
