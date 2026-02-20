#[allow(unused)]
use clap::error::ErrorKind as ClapErrorKind;
use clap::Parser;
use core::todo;
use log::{debug, error, info};
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use Commands::*;

mod cmd;
mod git;
mod records;
mod utils;

use cmd::*;
use git::*;
use records::*;
use records::Progress;
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
    // get .trans dir
    let trans_dir = get_trans_dir();
    // get records.toml file
    let records_toml = get_records_toml();

    match &cli.command {
        // init .trans folder
        Init { lang, tag } => {
            let content = toml::to_string(&Records::init(lang, tag).unwrap()).unwrap();
            create_file_with_dirs(records_toml)
                .and_then(|mut file| {
                    info!("File .trans/records.toml created.");
                    file.write_all(content.as_bytes());
                    Ok(())
                })
                .or_else(|err| {
                    error!("File .trans/records.toml created failed.");
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
                Diff { path_args: path, gendiff } => {
                    let path = &path[0].to_path_buf();
                    let old_rev = records.get(path).unwrap().track_rev;
                    let new_rev = get_file_rev(path);
                    let diff_file = get_diff(path, &old_rev, &new_rev);
                    if *gendiff {
                        write_diff_file_to_trans(path, &diff_file)?;
                    } else {
                        println!("{}", diff_file);
                    }
                    Ok(())
                }
                Cover => {
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
                Status => {
                    for prog in Progress::iter() {
                        records.show_progress(prog);
                    }
                    Ok(())
                }
                Show { status } => {
                    match status {
                        ShowStatus::All => records.show_all(),
                        ShowStatus::Trans => records.show_progress(Progress::Trans),
                        ShowStatus::Review => records.show_progress(Progress::Review),
                        ShowStatus::Done => records.show_progress(Progress::Done),
                        ShowStatus::Synced => records.show_synced(true),
                        ShowStatus::Unsynced => records.show_synced(false),
                        ShowStatus::Locked => records.show_locked(true),
                        ShowStatus::Unlocked => records.show_locked(false),
                    }
                    Ok(())
                }
                Mark { status } => {
                    match status {
                        MarkProgress::Trans { path } => { records.mark_progress(Progress::Trans, path)?; }
                        MarkProgress::Review { path } => { records.mark_progress(Progress::Review, path)?; }
                        MarkProgress::Done { path } => { records.mark_progress(Progress::Done, path)?; }
                    }
                    Ok(())
                }
                Sync { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.set_synced(path);
                    Ok(())
                }
                Update => {
                    records.update_sync();
                    Ok(())
                }
                Lock { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.set_lock(true, path)?;
                    Ok(())
                }
                Unlock { path_args: path } => {
                    let path = &path[0].to_path_buf();
                    records.set_lock(false, path)?;
                    Ok(())
                }

                _ => todo!(),
            }
        }
    }
}
