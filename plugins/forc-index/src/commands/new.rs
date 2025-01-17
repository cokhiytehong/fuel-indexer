use crate::ops::forc_index_new;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Create a new indexer project in a new directory.
#[derive(Debug, Parser)]
pub struct Command {
    /// Path at which to create indexer.
    pub path: PathBuf,

    /// Name of indexer.
    #[clap(long, help = "Name of indexer.")]
    pub name: Option<String>,

    /// Namespace to which indexer belongs.
    #[clap(long, help = "Namespace to which indexer belongs.")]
    pub namespace: Option<String>,

    /// Initialize an indexer with native execution enabled.
    #[clap(long, help = "Initialize an indexer with native execution enabled.")]
    pub native: bool,

    /// Resolve indexer asset filepaths using absolute paths.
    #[clap(long, help = "Resolve indexer asset filepaths using absolute paths.")]
    pub absolute_paths: bool,

    /// Enable verbose output.
    #[clap(short, long, help = "Enable verbose output.")]
    pub verbose: bool,
}

pub fn exec(command: Command) -> Result<()> {
    forc_index_new::init(command)?;
    Ok(())
}
