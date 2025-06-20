use clap::{Parser, ArgGroup};
use std::path::PathBuf;

/// Args to PROB problem
#[derive(Debug, Parser)]
#[command(
    group(
        ArgGroup::new("input")
            .required(true)
            .args(["readfile"])
    )
)]
pub struct ProbArgs {
    /// Input the file.
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,

    /// Print results to stdout (default: false).
    #[arg(long, default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub printresult: bool,
}