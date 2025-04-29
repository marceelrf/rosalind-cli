use clap::Args;
use std::path::PathBuf;

/// Args to GC problem
#[derive(Debug, Args)]
#[clap(group(
    clap::ArgGroup::new("input")
        .required(true)
        .args(&["fasta"]),
))]
pub struct GcArgs {

    /// Path to FASTA file
    #[clap(short = 'f', long)]
    pub fasta: PathBuf,

    /// Filename to be written.
    #[clap(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}