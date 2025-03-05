#[allow(unused)]
use clap::error::ErrorKind as ClapErrorKind;
use clap::{Args, FromArgMatches, Parser, Subcommand};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

mod cmd;
mod git;
mod records;
mod utils;

use cmd::*;
use git::*;
use records::*;
use utils::*;

#[allow(unused)]
pub fn main() -> Result<(), Error> {
    // initial logger and cli parser
    env_logger::init();
    let cli = Cli::parse();

    // check if this is a git repo
    if get_root_dir().is_none() {
        let not_a_repo = Error::new(ErrorKind::NotFound, "this is not a repo");
        return Err(not_a_repo);
    }

    // get repo dir
    let root_dir: PathBuf = get_root_dir().unwrap();
    debug!("repo dir: {:?}", root_dir);

    // get .trans dir
    let trans_dir = get_trans_dir();
    debug!("trans dir: {:?}", trans_dir);

    // get records.toml file
    let records_toml = get_records_toml();
    debug!("records_toml file: {:?}", records_toml);

    match &cli.command {
        // init .trans folder
        Commands::Init => {
            info!("'git trans init' was run");
            fs::create_dir_all(trans_dir).and_then(|_| {
                info!("目录 .trans 创建成功");
                Ok(())
            });
            File::create_new(records_toml)
                .and_then(|mut file| {
                    info!("文件 .trans/records.toml 创建成功");
                    let content = toml::to_string(&Records::init()).unwrap();
                    file.write_all(content.as_bytes());
                    Ok(())
                })
                .or_else(|err| {
                    error!("文件 .trans/records.toml 创建失败");
                    Err(err)
                })
        }

        _ => {
            // check if records.toml exists
            if !Path::new(&records_toml).is_file() {
                eprintln!("this repo is not initialized with git-trans");
                let not_exsist = Error::new(
                    ErrorKind::NotFound,
                    "this repo is not initialized with git-trans",
                );
                return Err(not_exsist);
            }

            let mut records_str = fs::read_to_string(&records_toml)?;
            let mut records : Records = toml::from_str(&records_str).unwrap();

            match &cli.command {
                // add files to .trans
                Commands::Add(path_args) => {
                    let path = path_args.path.as_ref().unwrap().to_path_buf();
                    let path_rel_to_root = absolute_to_relative::<PathBuf, PathBuf>(
                        root_dir,
                        relative_to_absolute(path.clone()).unwrap(),
                    )
                    .unwrap();
                    let added_file = records.add(path.clone(), false).unwrap();
                    records_str = toml::to_string(&records).unwrap();
                    fs::write(&records_toml, records_str).unwrap();

                    debug!(
"'git trans add' was run,\n
path: {}\n
toml:\n
{}",
                        path.display(),
                        toml::to_string(&added_file).unwrap());
                    Ok(())
                }
                Commands::Rm(path_args) => {
                    debug!(
                        "'git trans rm' was run, name is: {:?}",
                        path_args.path.as_ref().unwrap()
                    );
                    Ok(())
                }
                Commands::Check(path_args) => {
                    debug!(
                        "'git trans check' was run, name is: {:?}",
                        path_args.path.as_ref().unwrap()
                    );
                    Ok(())
                }
                Commands::Diff(path_args) => {
                    debug!(
                        "'git trans diff' was run, name is: {:?}",
                        path_args.path.as_ref().unwrap()
                    );
                    Ok(())
                }
                Commands::Gendiff(path_args) => {
                    debug!(
                        "'git trans gendiff' was run, name is: {:?}",
                        path_args.path.as_ref().unwrap()
                    );
                    Ok(())
                }
                Commands::Sync(path_args) => {
                    debug!(
                        "'git trans sync' was run, name is: {:?}",
                        path_args.path.as_ref().unwrap()
                    );
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }
}
