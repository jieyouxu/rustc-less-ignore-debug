use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use miette::{bail, Context, Diagnostic, IntoDiagnostic, Result, Severity};
use thiserror::Error;
use tracing::*;

use crate::config::Config;

/// Run the reduction steps.
///
/// For each of the tests in the specified directories / suites:
/// - Run the unmodified test as a sanity check
/// - (CASE remove-directives) Remove `// ignore-debug`, try to run the test and see if it passes
///   (assuming it is no longer ignored). If it passes, then we can keep the changes. Otherwise,
///   restore the original test.
/// - (CASE replace-directives) Try to specify the compile flags directive
///   `// compile-flags: -Cdebug-assertions=no`, try to run the test and see it passes. If it
///   passes, keep the changes, otherwise, revert.
/// At the end of the run, generate a summary / report detailing, for each changed test, what
/// specifically has been done (either remove directive entirely or replace directive).
pub fn run(
    config: &Config,
    current_exe_path: &Path,
    rustc_repo_path: &Path,
    report_path: Option<&Path>,
) -> Result<()> {
    debug!(
        ?config,
        ?rustc_repo_path,
        ?report_path,
        "run command invoked"
    );

    if !rustc_repo_path.exists() {
        bail!(
            "`{}` does not exist, please check your path to rustc repo",
            rustc_repo_path.display()
        );
    }

    if config.target_directories.is_empty() {
        warn!("no target directories specified in config");
        warn!("maybe you forgot to edit the config?");
        bail!(
            severity = Severity::Warning,
            "no target directories specified, exiting"
        );
    }

    // Let's check if bootstrap `x` is available and executable.
    {
        match Command::new("x").output() {
            Ok(_) => {
                info!("detected bootstrap script `x`");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                error!(
                    "could not detect bootstrap `x`, did you provide a correct rustc repo path?"
                );
            }
            Err(e) => Err(e)
                .into_diagnostic()
                .wrap_err("error while trying to detect bootstrap script `x`")?,
        }
    }

    // Let's check if all of the specified target directories exist for early reporting.
    for p in &config.target_directories {
        let path = rustc_repo_path.join(p);
        if !path.exists() {
            bail!("target directory `{}` does not exist", path.display());
        }
    }

    let mut target_files = BTreeSet::new();

    trace!("iter through target directories");
    for p in &config.target_directories {
        let dir = rustc_repo_path.join(p);
        trace!(?dir);

        let iter = walkdir::WalkDir::new(dir)
            .sort_by_file_name()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                !e.file_type().is_dir()
                    && e.path()
                        .extension()
                        .map(|s| s == "rs" || s == "fixed")
                        .unwrap_or(false)
            })
            .map(|e| e.into_path());
        target_files.extend(iter);
    }

    info!(
        "there are {} target test files to be processed",
        target_files.len()
    );

    let mut report: BTreeMap<PathBuf, RunOutcome> = BTreeMap::new();

    trace!("processing each file");
    for target_file in &target_files {
        trace!(?target_file);
        let outcome = try_run(target_file)?;
        report.insert(target_file.to_path_buf(), outcome);
    }

    let report = format_report(&report);

    let report_path = current_exe_path.join("report.md");
    std::fs::write(&report_path, report)
        .into_diagnostic()
        .wrap_err(format!(
            "failed to write report to {}",
            report_path.display()
        ))?;
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]

enum RunOutcome {
    /// The test needs to remain unmodified because removal or replacement of `// ignore-debug`
    /// both cause errors.
    UnmodifiedOk,
    /// The test has its `// ignore-debug` directive removed and still passes.
    RemoveOk,
    /// The test has its `// ignore-debug` directive removed, but needs
    /// `// compile-flags: -Cdebug-assertions=no` to pass.
    ReplaceOk,
    /// The test is ignored.
    Ignored,
}

fn try_run(target: &Path) -> miette::Result<RunOutcome> {
    sanity_check(target)?;

    match try_remove(target) {
        Ok(RunOutcome::Ignored) => return Ok(RunOutcome::Ignored),
        Ok(_) => {}
        Err(e) if matches!(e, RunError::TestFailure) => {
            return Ok(RunOutcome::UnmodifiedOk);
        }
        Err(e) => Err(e)?,
    }

    match try_replace(target) {
        Ok(RunOutcome::Ignored) => Ok(RunOutcome::Ignored),
        Ok(_) => Ok(RunOutcome::ReplaceOk),
        Err(e) if matches!(e, RunError::TestFailure) => Ok(RunOutcome::RemoveOk),
        Err(e) => Err(e)?,
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("run error")]
enum RunError {
    /// We successfully invoked `./x test <path-to-test-file> --stage 1`, but the test failed.
    #[error("test failed")]
    TestFailure,
    /// Some other unexpected kind of error.
    #[error("unexpected error")]
    Other(miette::Error),
}

// `./x test <path-to-test-file> --stage 1 --bless`
fn invoke_x(rustc_repo_path: &Path, target: &Path) -> miette::Result<Output> {
    Command::new("x")
        .current_dir(rustc_repo_path)
        .arg("test")
        .arg(target)
        .arg("--stage")
        .arg("1")
        .arg("--bless")
        .output()
        .into_diagnostic()
        .wrap_err(format!(
            "error trying to invoke `x test {} --stage 1`",
            target.display()
        ))
}

/// Run the unmodified test as a sanity check
fn sanity_check(_target: &Path) -> miette::Result<RunOutcome, RunError> {
    todo!()
}

/// Remove `// ignore-debug`, try to run the test and see if it passes (assuming it is no longer
/// ignored). If it passes, then we can keep the changes. Otherwise, restore the original test.
fn try_remove(_target: &Path) -> miette::Result<RunOutcome, RunError> {
    todo!()
}

/// Try to replace `// ignore-debug` by the compile flags directive
/// `// compile-flags: -Cdebug-assertions=no`, try to run the test and see it passes. If it
/// passes, keep the changes, otherwise, revert.
fn try_replace(_target: &Path) -> miette::Result<RunOutcome, RunError> {
    todo!()
}

fn format_report(_report: &BTreeMap<PathBuf, RunOutcome>) -> String {
    todo!()
}
