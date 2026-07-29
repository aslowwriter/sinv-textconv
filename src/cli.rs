use std::path::PathBuf;

use clap::Parser;

/// a git `textconv` program for Sphinc objects.inv files
#[derive(Parser, Debug)]
#[command(version, about)]
pub(crate) struct CliArgs {
    /// Path to the file to convert to plain text
    pub file: PathBuf,
}
