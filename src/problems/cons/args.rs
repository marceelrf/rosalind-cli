// cons - args.rs
use clap::{Parser, ArgGroup};
use std::path::PathBuf;

/// Args to SPLC problem
#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("input")
            .required(true)
            .args(["readfile"])
    )
)]
pub struct ConsArgs {
    /// Input a fasta file.
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}