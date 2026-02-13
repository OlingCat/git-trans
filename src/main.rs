#[allow(unused)]
use clap::error::ErrorKind as ClapErrorKind;
use clap::Parser;
use core::todo;
use log::{debug, error, info};
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use Commands::Status as Cmdstatus;
use Commands::*;

mod cmd;
mod git;
mod records;
mod utils;

use cmd::*;
use git::*;
use records::*;
use records::Status;
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
        Init { lang, tag } => {
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
                Add { path_args: path, lock } => {
                    let path = path[0].to_path_buf();
                    let path_rel_to_root = get_path_rel_to_root(&path);

                    copy_file_to_trans(&path)?;

                    let added_file = records.add(&path, *lock)?;
                    records_str = toml::to_string(&records).unwrap();
                    fs::write(&records_toml, records_str)?;
                    Ok(())
                }
                Rm { path_args: path } => {
                    let removed_file = records.remove(&path[0].to_path_buf()).unwrap();
                    records_str = toml::to_string(&records).unwrap();
                    fs::write(&records_toml, records_str).unwrap();
                    Ok(())
                }
                Ls {
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
                Diff { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    let old_rev = records.get(path).unwrap().track_rev;
                    let new_rev = get_file_revision(path);
                    println!("{}", get_diff(path, &old_rev, &new_rev));
                    Ok(())
                }
                Gendiff { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    let old_rev = records.get(path).unwrap().track_rev;
                    let new_rev = get_file_revision(path);
                    let diff_file = get_diff(path, &old_rev, &new_rev);
                    // this will write diff file to .trans dir
                    write_diff_file_to_trans(path, &diff_file)?;
                    Ok(())
                }
                Sync { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.sync(path);
                    Ok(())
                }
                Lock { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.lock(path);
                    Ok(())
                }
                Unlock { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.unlock(path);
                    Ok(())
                }
                Cover => {
                    debug!("copy files in .trans dir to root dir");
                    let count = cover()?;
                    Ok(())
                }
                Reset => {
                    reset();
                    Ok(())
                }
                Log => {
                    println!("{}", get_log(&get_trans_dir()));
                    Ok(())
                }
                Cmdstatus => {
                    debug!("'git trans status' was run");
                    for status in Status::iter() {
                        records.show(status);
                    }
                    Ok(())
                }
                Show { status } => {
                    match status {
                        Status::Todo => records.show(Status::Todo),
                        Status::ToReview => records.show(Status::ToReview),
                        Status::Done => records.show(Status::Done),
                        Status::Unsynced => records.show(Status::Unsynced),
                        Status::Synced => records.show(Status::Synced),
                    }
                    Ok(())
                }
                Mark { status } => {
                    match status {
                        cmd::CmdStatus::Todo { path } => { records.mark(Status::Todo, path)?; }
                        cmd::CmdStatus::ToReview { path } => { records.mark(Status::ToReview, path)?; }
                        cmd::CmdStatus::Done { path } => { records.mark(Status::Done, path)?; }
                        cmd::CmdStatus::Unsynced { path } => { records.mark(Status::Unsynced, path)?; }
                        cmd::CmdStatus::Synced { path } => { records.mark(Status::Synced, path)?; }
                    }
                    Ok(())
                }
                _ => todo!(),
            }
        }
    }
}
