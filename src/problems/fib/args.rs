use clap::{Args, ArgGroup};
use std::path::PathBuf;

/// Args to FIB problem
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["readfile", "n"])
))]
pub struct FibArgs {
    /// Input file containing n and k values (format: "n k")
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// Number of months (must be used with -k)
    #[arg(short = 'n', long, requires = "k")]
    pub n: Option<u32>,

    /// Number of offspring pairs per reproduction (must be used with -n)
    #[arg(short = 'k', long)]
    pub k: Option<u32>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,
} // /mnt/c/Users/madra/Downloads/