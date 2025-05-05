use clap::{Args, ArgGroup};
use std::path::PathBuf;

/// Args to IPRB problem
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["readfile", "k"])
))]
pub struct IprbArgs {
    /// Input file containing k,m and n values (format: "k m n")
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,

    /// Individuals are homozygous dominant for a factor (must be used with -n and -m)
    #[arg(short = 'k', long)]
    pub k: Option<u32>,

    /// Heterozygous individuals (must be used with -k and -n)
    #[arg(short = 'm', long, requires = "k", requires = "n")]
    pub m: Option<u32>,

    /// Homozygous recessive individuals (must be used with -k and -m)
    #[arg(short = 'n', long, requires = "k", requires = "m")]
    pub n: Option<u32>,

    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,
} 