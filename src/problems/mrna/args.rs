use clap::{Args, ArgGroup};
use std::path::PathBuf;

/// Args to MRNA problem
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["readfile"])
))]
pub struct MrnaArgs {
    /// Input file containing the protein sequence
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,
    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}