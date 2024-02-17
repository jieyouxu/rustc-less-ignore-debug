#![feature(let_chains)]

mod cli;
mod config;
mod logging;
mod run;

use clap::Parser as _;
use confique::toml::FormatOptions;
use confique::Config as _;
use miette::{bail, miette, Context, IntoDiagnostic, Severity};
use tracing::*;

use crate::cli::{Cli, Command};
use crate::config::Config;

const TARGET_TRIPLE: &str = env!("TARGET");

fn main() -> miette::Result<()> {
    logging::setup_logging();

    let cli = Cli::parse();
    debug!(?cli);

    let exe_path = std::env::current_exe().into_diagnostic()?;
    let config_path = exe_path.parent().unwrap().join("config.toml");
    debug!(?config_path);
    debug!("config exists: {}", config_path.exists());
    let config = if cli.command != Command::GenerateConfig {
        info!("trying to read config from `{}`", config_path.display());
        if !config_path.exists() {
            info!("no existing config detected");
            info!("you can generate a default config via `generate-config` command");
            info!("the tool will now exit");
            return Ok(());
        }

        let config = Config::from_file(&config_path)
            .inspect_err(|e| {
                warn!("failed to load config from `{}`", config_path.display());
                warn!("default config values will be used");
                warn!(?e);
            })
            .unwrap_or_default();
        debug!(?config);
        config
    } else {
        Config::default()
    };

    match &cli.command {
        Command::GenerateConfig => {
            if !config_path.exists() {
                info!("generating config at `{}`", config_path.display());
                let template = confique::toml::template::<Config>(FormatOptions::default());
                std::fs::write(&config_path, template).into_diagnostic()?;
            } else {
                error!("config.toml already exists");
                bail!("config.toml already exists!");
            }
        }
        Command::Run { rustc_repo_path } => {
            run::run(&config, rustc_repo_path.as_path())?;
        }
    }

    if config.target_directories.is_empty() {
        warn!("no target directories specified in config");
        warn!("maybe you forgot to edit the config?");
        bail!(
            severity = Severity::Warning,
            "no target directories specified, exiting"
        );
    }

    Ok(())
}
