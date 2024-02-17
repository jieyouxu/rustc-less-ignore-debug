use std::path::Path;

use miette::{bail, Result, Severity};
use tracing::*;

use crate::config::Config;

pub fn run(config: &Config, _rustc_repo_path: &Path) -> Result<()> {
    if config.target_directories.is_empty() {
        warn!("no target directories specified in config");
        warn!("maybe you forgot to edit the config?");
        bail!(
            severity = Severity::Warning,
            "no target directories specified, exiting"
        );
    }

    todo!()
}
