use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{io::Error, io::ErrorKind, path::PathBuf, str::FromStr};
use toml::value::Datetime;

use crate::{git::*, utils::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Records {
    pub meta: Meta,
    pub files: Vec<TrackedFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub project_name: String,
    pub base_hash: String,
    pub datetime: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackedFile {
    pub path: PathBuf,
    pub track_hash: String,
    pub locked: Option<bool>,
}

impl Records {
    pub fn init() -> Records {
        let root_dir: PathBuf = get_root_dir().unwrap();
        return Records {
            meta: Meta {
                project_name: root_dir.file_name().unwrap().to_str().unwrap().to_string(),
                base_hash: get_head_hash(),
                datetime: Datetime::from_str(&Utc::now().to_rfc3339()).unwrap(),
            },
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, path: PathBuf, locked: bool) -> Result<TrackedFile, Error> {
        let root_dir: PathBuf = get_root_dir().unwrap();
        let path = pathbuf_to_unix_style(&path);
        if self.contains(&path) {
            let err = Error::new(ErrorKind::AlreadyExists, "file already exists");
            return Err(err);
        }

        let path_rel_to_root = pathbuf_to_unix_style(
            &absolute_to_relative::<PathBuf, PathBuf>(
                root_dir,
                relative_to_absolute(path.clone()).unwrap()).unwrap());
        let file = TrackedFile {
            path: path_rel_to_root,
            track_hash: get_file_revision(&path),
            locked: if locked { Some(true) } else { None },
        };
        self.files.push(file.clone());
        return Ok(file);
    }

    pub fn rm(&mut self, path: &PathBuf) -> Result<TrackedFile, Error> {
        let root_dir: PathBuf = get_root_dir().unwrap();
        let path = pathbuf_to_unix_style(&path);
        let path_rel_to_root = pathbuf_to_unix_style(
            &absolute_to_relative::<PathBuf, PathBuf>(
                root_dir,
                relative_to_absolute(path.clone()).unwrap()).unwrap());

    }

    pub fn contains(&self, path: &PathBuf) -> bool {
        let root_dir: PathBuf = get_root_dir().unwrap();
        let path = pathbuf_to_unix_style(&path);
        let path_rel_to_root = pathbuf_to_unix_style(
            &absolute_to_relative::<PathBuf, PathBuf>(
                root_dir,
                relative_to_absolute(path.clone()).unwrap()).unwrap());
        self.files.iter().any(|file| file.path == path_rel_to_root)
    }
}
