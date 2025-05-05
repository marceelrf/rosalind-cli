use clap::{Args, ArgGroup};
use std::path::PathBuf;

/// Args to IEV problem
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["readfile"])
))]
pub struct IevArgs {
    /// Input file containing SIX integers correspond to the number of couples in a population possessing each genotype pairing for a given factor.
    #[arg(short = 'f', long)]
    pub readfile: Option<PathBuf>,
    /// Output file to write results
    #[arg(short = 'w', long)]
    pub writefile: Option<PathBuf>,

    
}