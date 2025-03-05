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
    Init,
    /// Add files to .trans
    Add(PathArgs),
    /// Remove files from .trans
    Rm(PathArgs),
    /// Check if the file is synchronized
    Check(PathArgs),
    /// Diff file changes
    Diff(PathArgs),
    /// Generate diff file
    Gendiff(PathArgs),
    /// Sync file with latest revision
    Sync(PathArgs),
}

#[derive(Args)]
pub struct PathArgs {
    pub path: Option<PathBuf>,
}
