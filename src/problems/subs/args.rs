use clap::Args;
use std::path::PathBuf;

/// Args to GC problem
#[derive(Debug, Args)]
#[clap(group(
    clap::ArgGroup::new("input")
        .required(true)
        .args(&["readfile"]),
))]
pub struct SubsArgs {

    /// Path to FASTA file
    #[clap(short = 'f', long)]
    pub readfile: PathBuf,

    /// Filename to be written.
    #[clap(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}