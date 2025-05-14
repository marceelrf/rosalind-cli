use clap::{Parser, ArgGroup};
use std::path::PathBuf;

/// Args to MPRT problem
#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("input")
            .required(true)
            .args(["readfile"])
    )
)]
pub struct MprtArgs {
    /// Input file containing the uniprot ids (One per row)
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}
