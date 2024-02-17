use confique::Config as DeriveConfig;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Default, DeriveConfig)]
pub struct Config {
    /// `rustc` test directories to perform the attempted reduction of `// ignore-debug` for.
    /// They need to be paths relative to the root of the `rustc` repo, e.g. `tests/run-make`.
    #[config(default = [])]
    pub target_directories: BTreeSet<PathBuf>,
}
