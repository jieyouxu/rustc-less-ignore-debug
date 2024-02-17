use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Generate a default config file in the same directory as the executable.
    GenerateConfig,
    /// Run the tool on the specified directories under the given `rustc` repo.
    Run {
        /// Path to the `rustc` repo.
        rustc_repo_path: PathBuf,
    },
}
