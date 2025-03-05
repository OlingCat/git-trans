use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use toml::value::Datetime;

use crate::{git::*, utils::pathbuf_to_unix_style};

#[derive(Debug, Serialize, Deserialize)]
pub struct Records {
    meta: Meta,
    files: Option<Vec<File>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    project_name: String,
    base_hash: String,
    datetime: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    file_name: String,
    path: PathBuf,
    track_hash: String,
    locked: Option<bool>,
}

pub fn init_records(repo_dir: PathBuf) -> String {
    let record = Records {
        meta: Meta {
            project_name: repo_dir.file_name().unwrap().to_str().unwrap().to_string(),
            base_hash: get_head_hash(),
            datetime: Datetime::from_str(&Utc::now().to_rfc3339()).unwrap(),
        },
        files: None,
    };
    return toml::to_string(&record).unwrap();
}

pub fn add_file(path: PathBuf, locked: bool) -> String {
    let file = File {
        file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
        path: pathbuf_to_unix_style(&path),
        track_hash: get_file_revision(&path),
        locked: if locked { Some(true) } else { None },
    };
    return toml::to_string(&file).unwrap();
}
