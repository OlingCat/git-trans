use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::records::Status;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Init .trans folder
    #[command(arg_required_else_help = true)]
    Init {
        /// Language of the project
        #[arg(required = true)]
        lang: String,
        /// Lock track revision
        #[arg(default_value = "HEAD")]
        tag: String,
    },
    /// Add files to .trans
    #[command(arg_required_else_help = true)]
    Add {
        /// Files to add to the records
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
        /// Lock files
        #[arg(long)]
        lock: bool,
    },
    /// Remove files from .trans
    #[command(arg_required_else_help = true)]
    Rm {
        /// Files to remove from the records
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// List files in .trans
    Ls {
        /// Path to list files in
        #[arg(default_value = "./")]
        path: Option<PathBuf>,
        /// List files recursively
        #[arg(short, long)]
        recursive: bool,
        /// List all files recorded
        #[arg(short, long, exclusive = true)]
        all: bool,
    },
    /// Check if the file is synchronized
    Status,
    /// Diff file changes
    #[command(arg_required_else_help = true)]
    Diff {
        /// File to diff with latest revision
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Generate diff file
    #[command(arg_required_else_help = true)]
    Gendiff {
        /// File to generate diff with latest revision
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Sync file with latest revision
    #[command(arg_required_else_help = true)]
    Sync {
        /// Files to sync with latest revision
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Lock file
    Lock {
        /// Files to lock
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Unlock file
    Unlock {
        /// Files to unlock
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Cover files in repo root folder
    Cover,
    /// Reset the root folder to the latest revision
    Reset,
    /// Show logs in the .trans folder
    Log,
    /// Show files in a specific state
    Show {
        /// Action to perform on the files
        #[command(subcommand)]
        status: Status,
    },
    /// Mark files as given status
    #[command(arg_required_else_help = true)]
    Mark {
        /// Action to perform on the files
        #[command(subcommand)]
        status: CmdStatus,
    },
}

#[derive(Subcommand)]
pub enum CmdStatus {
    Todo {
        path: PathBuf,
    },
    ToReview {
        path: PathBuf,
    },
    Done {
        path: PathBuf,
    },
    Unsynced {
        path: PathBuf,
    },
    Synced {
        path: PathBuf,
    },
}

impl From<CmdStatus> for Status {
    fn from(cmd_status: CmdStatus) -> Self {
        match cmd_status {
            CmdStatus::Todo { .. } => Status::Todo,
            CmdStatus::ToReview { .. } => Status::ToReview,
            CmdStatus::Done { .. } => Status::Done,
            CmdStatus::Unsynced { .. } => Status::Unsynced,
            CmdStatus::Synced { .. } => Status::Synced,
        }
    }
}


#[derive(Args)]
pub struct PathArgs {
    pub path: Option<PathBuf>,
}
