use confique::Config as DeriveConfig;
use std::collections::BTreeSet;

#[derive(Debug, Default, DeriveConfig)]
pub struct Config {
    /// `rustc` test directories to perform the attempted reduction of `// ignore-debug` for.
    #[config(default = [])]
    pub target_directories: BTreeSet<String>,
}
