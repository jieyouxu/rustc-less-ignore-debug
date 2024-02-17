use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Cmd,
}

#[derive(Debug, PartialEq, Subcommand)]
pub(crate) enum Cmd {
    /// Generate a default config file in the same directory as the executable.
    GenerateConfig,
    /// Run the tool on the specified directories under the given `rustc` repo.
    Run {
        /// Path to the `rustc` repo.
        rustc_repo_path: PathBuf,
        /// Path to generate the run report. If not specified, will default to `run_summary.md`
        /// under the same directory as the executable.
        report_path: Option<PathBuf>,
    },
}
