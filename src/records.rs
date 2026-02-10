use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
};
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
    pub lang: String,
    pub track_rev: String,
    pub datetime: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackedFile {
    pub path: PathBuf,
    pub track_rev: String,
    pub locked: Option<bool>,
}

impl Records {
    pub fn init(lang: &String, tag: &String) -> Result<Records, Error> {
        let root_dir: PathBuf = get_root_dir().unwrap();
        let project_name = root_dir.file_name().unwrap().to_str().unwrap().to_string();
        if let Some(rev) = get_revision(tag) {
            return Ok(Records {
                meta: Meta {
                    project_name: project_name,
                    lang: lang.clone(),
                    track_rev: if tag == "HEAD" { rev } else { tag.clone() },
                    datetime: Datetime::from_str(&Utc::now().to_rfc3339()).unwrap(),
                },
                files: Vec::new(),
            });
        } else {
            let err = Error::new(ErrorKind::InvalidInput, format!("{tag} is not a valid revision"));
            return Err(err);
        }
    }

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
            locked: if lock { Some(true) } else { None },
        };
        self.files.push(file.clone());
        return Ok(file);
    }

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

    pub fn contains(&self, path: &PathBuf) -> bool {
        let path = unify(&path);
        let path_rel_to_root = get_path_rel_to_root(&path);
        self.files.iter().any(|file| file.path == path_rel_to_root)
    }
}
