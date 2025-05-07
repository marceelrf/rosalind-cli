use clap::{Args, ArgGroup};
use std::path::PathBuf;

/// Args to LIA problem
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["readfile", "n"])
))]

pub struct LiaArgs {
    /// Input file containing two integers correspond to N and K. The probability that at least N Aa Bb organisms will belong to the k-th generation of Tom's family tree (don't count the Aa Bb mates at each level
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,
    /// Number of months (must be used with -k)
    #[arg(short = 'n', long, requires = "k")]
    pub n: Option<u64>,
    /// Number of offspring pairs per reproduction (must be used with -n)
    #[arg(short = 'k', long)]
    pub k: Option<u64>,
    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,    
}