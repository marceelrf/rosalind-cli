use clap::{Args, ArgGroup};
use std::path::PathBuf;

/// Args to FIBD (Mortal Fibonacci Rabbits) problem
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["readfile", "n"])
))]

pub struct FibdArgs {
    /// Input file containing n and m values (format: "n m")
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// The total number of months (n)
    #[arg(short = 'n', requires = "m")]
    pub n: Option<u64>,

    /// Number of months that rabbits survive (m)
    #[arg(short = 'm', requires = "n")]
    pub m: Option<u64>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}