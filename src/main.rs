#[allow(unused)]
use clap::error::ErrorKind as ClapErrorKind;
use clap::Parser;
use core::todo;
use log::{debug, error, info};
use std::fs;
use std::io::{Error, ErrorKind, Write};
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
        let not_a_repo = Error::new(ErrorKind::NotFound, "This is not a repo");
        return Err(not_a_repo);
    }

    // get repo dir
    let root_dir: PathBuf = get_root_dir().unwrap();
    debug!("repo dir: {:?}", root_dir);

    // get .trans dir
    let trans_dir = get_trans_dir();
    debug!(".trans dir: {:?}", trans_dir);

    // get records.toml file
    let records_toml = get_records_toml();
    debug!("records.toml: {:?}", records_toml);

    match &cli.command {
        // init .trans folder
        Commands::Init { lang, tag } => {
            info!("'git trans init' was run");
            let content = toml::to_string(&Records::init(lang, tag).unwrap()).unwrap();
            create_file_with_dirs(records_toml)
                .and_then(|mut file| {
                    info!("文件 .trans/records.toml 创建成功");
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
            let mut records: Records = toml::from_str(&records_str).unwrap();

            match &cli.command {
                Commands::Add { path_args: path, lock } => {
                    let path = path[0].to_path_buf();
                    let path_rel_to_root = get_path_rel_to_root(&path);

                    copy_file_to_trans(&path)?;

                    let added_file = records.add(&path, *lock)?;
                    records_str = toml::to_string(&records).unwrap();
                    fs::write(&records_toml, records_str)?;

                    debug!(
                        "'git trans add' was run,\npath: {}\nlocked: {}\ntoml:\n{}",
                        path.display(),
                        lock,
                        toml::to_string(&added_file).unwrap(),
                    );
                    Ok(())
                }
                Commands::Rm { path_args: path } => {
                    let removed_file = records.remove(&path[0].to_path_buf()).unwrap();
                    records_str = toml::to_string(&records).unwrap();
                    fs::write(&records_toml, records_str).unwrap();

                    debug!(
                        "'git trans rm' was run,\npath: {}\ntoml:\n{}",
                        path[0].to_path_buf().display(),
                        toml::to_string(&removed_file).unwrap()
                    );
                    Ok(())
                }
                Commands::Ls {
                    path,
                    all,
                    recursive,
                } => {
                    debug!(
                        "'git trans ls' was run,\npath: {}\nall: {}",
                        path.as_ref().unwrap().to_path_buf().display(),
                        all
                    );
                    Ok(())
                }
                Commands::Status => {
                    debug!("'git trans status' was run");
                    Ok(())
                }
                Commands::Diff{ path_args: path } => {
                    let path = &path[0].to_path_buf();
                    let old_rev = records.get(path).unwrap().track_rev;
                    let new_rev = get_file_revision(path);
                    println!("{}", get_diff(path, &old_rev, &new_rev));

                    debug!(
                        "'git trans diff' was run,\npath: {}\ntoml:\n{}",
                        path.to_path_buf().display(),
                        "None"
                    );
                    Ok(())
                }
                Commands::Gendiff{ path_args: path } => {
                    let path = &path[0].to_path_buf();
                    let old_rev = records.get(path).unwrap().track_rev;
                    let new_rev = get_file_revision(path);
                    let diff_file = get_diff(path, &old_rev, &new_rev);
                    write_diff_file_to_trans(path, &diff_file)?;     // write diff file to .trans dir
                    debug!(
                        "'git trans gendiff' was run,\npath: {}\ntoml:\n{}",
                        path.to_path_buf().display(),
                        "None"
                    );
                    Ok(())
                }
                Commands::Sync{ path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.sync(path);
                    debug!(
                        "'git trans sync' was run,\npath: {}\ntoml:\n{}",
                        path.to_path_buf().display(),
                        "None"
                    );
                    Ok(())
                }
                Commands::Lock{ path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.lock(path);
                    debug!(
                        "'git trans lock' was run,\npath: {}\ntoml:\n{}",
                        path.to_path_buf().display(),
                        "None"
                    );
                    Ok(())
                }
                Commands::Unlock{ path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.unlock(path);
                    debug!(
                        "'git trans unlock' was run,\npath: {}\ntoml:\n{}",
                        path.to_path_buf().display(),
                        "None"
                    );
                    Ok(())
                }
                Commands::Cover => {
                    debug!("copy files in .trans dir to root dir");
                    let count = cover()?;
                    Ok(())
                }
                Commands::Reset => {
                    reset();
                    Ok(())
                }
                Commands::Log => {
                    println!("{}", get_log(&get_trans_dir()));
                    Ok(())
                }
                _ => todo!(),
            }
        }
    }
}
