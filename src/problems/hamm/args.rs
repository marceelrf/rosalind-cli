use clap::Args;
use std::path::PathBuf;

/// Args to HAMM problem
#[derive(Debug, Args)]
pub struct HammArgs {
    /// Path to the file with two DNA sequences
    #[clap(short = 'f', long)]
    pub readfile: PathBuf,

    /// Filename to be written
    #[clap(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}