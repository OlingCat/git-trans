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
        paths: Vec<PathBuf>,
        /// Lock files
        #[arg(long)]
        lock: bool,
    },
    /// Remove files from .trans
    #[command(arg_required_else_help = true)]
    Rm {
        /// Files to remove from the records
        #[arg(required = true)]
        paths: Vec<PathBuf>,
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
    Diff(PathArgs),
    /// Generate diff file
    #[command(arg_required_else_help = true)]
    Gendiff(PathArgs),
    /// Sync file with latest revision
    #[command(arg_required_else_help = true)]
    Sync(PathArgs),
    Cover,
    Reset,
    Log,
}

#[derive(Args)]
pub struct PathArgs {
    pub path: Option<PathBuf>,
}
