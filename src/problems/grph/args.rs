// grph - args.rs
use clap::{Parser, ArgGroup};
use std::path::PathBuf;

/// Args to GRPH problem
#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("input")
                .required(true)
                .args(["readfile"])
    )
)]
pub struct GrphArgs {
    /// Input a multi fasta file.
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,

    /// Print results to stdout (default: true).
    #[arg(long, default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub printresult: bool,

    /// Overlap size (default: 3).
    #[arg(short = 'k', long, default_value_t = 3)]
    pub overlapsize: usize,
}