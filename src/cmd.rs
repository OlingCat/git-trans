use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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
    /// Show todo, review, and unsynced files
    Status,
    /// Diff file changes
    #[command(arg_required_else_help = true)]
    Diff {
        /// Generate diff files
        #[arg(short, long)]
        gendiff: bool,
        /// File to diff with latest revision
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
    /// Update sync status for all files
    Update,
    /// Lock files in the records
    Lock {
        /// Files to lock
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Unlock files in the records
    Unlock {
        /// Files to unlock
        #[arg(required = true)]
        path_args: Vec<PathBuf>,
    },
    /// Cover the .trans folder into the repo folder
    Cover,
    /// Reset the root folder to the latest revision
    Reset,
    /// Show logs in the .trans folder
    Log,
    /// Show files in a given status
    #[command(arg_required_else_help = true)]
    Show {
        #[command(subcommand)]
        status: ShowStatus,
    },
    /// Mark files as given status
    #[command(arg_required_else_help = true)]
    Mark {
        #[command(subcommand)]
        status: MarkProgress,
    },
}

#[derive(Subcommand)]
pub enum ShowStatus {
    /// Show all files
    All,
    /// Show todo files
    Todo,
    /// Show review files
    Review,
    /// Show done files
    Done,
    /// Show synced files
    Synced,
    /// Show unsynced files
    Unsynced,
    /// Show locked files
    Locked,
    /// Show unlocked files
    Unlocked,
}

#[derive(Subcommand)]
pub enum MarkProgress {
    /// Mark file as todo
    Todo {
        path: PathBuf,
    },
    /// Mark file as review
    Review {
        path: PathBuf,
    },
    /// Mark file as done
    Done {
        path: PathBuf,
    },
}

#[derive(Args)]
pub struct PathArgs {
    /// Path to the file
    pub path: Option<PathBuf>,
}
