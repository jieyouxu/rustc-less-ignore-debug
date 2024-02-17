#![feature(let_chains)]

mod cli;
mod config;
mod logging;
mod run;

use std::path::PathBuf;

use clap::Parser as _;
use confique::toml::FormatOptions;
use confique::Config as _;
use miette::{bail, IntoDiagnostic};
use tracing::*;

use crate::cli::{Cli, Cmd};
use crate::config::Config;

#[allow(dead_code)]
const TARGET_TRIPLE: &str = env!("TARGET");

fn main() -> miette::Result<()> {
    logging::setup_logging();

    let cli = Cli::parse();
    debug!(?cli);

    let exe_path = std::env::current_exe().into_diagnostic()?;
    let config_path = exe_path.parent().unwrap().join("config.toml");
    debug!(?config_path);
    debug!("config exists: {}", config_path.exists());
    let config = if cli.command != Cmd::GenerateConfig {
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
        Cmd::GenerateConfig => {
            if !config_path.exists() {
                info!("generating config at `{}`", config_path.display());
                let template = confique::toml::template::<Config>(FormatOptions::default());
                std::fs::write(&config_path, template).into_diagnostic()?;
            } else {
                error!("config.toml already exists");
                bail!("config.toml already exists!");
            }
        }
        Cmd::Run {
            rustc_repo_path,
            report_path,
        } => {
            run::run(
                &config,
                &exe_path,
                rustc_repo_path.as_path(),
                report_path.as_ref().map(PathBuf::as_path),
            )?;
        }
    }

    Ok(())
}
