use clap::Args;
use std::path::PathBuf;

/// Args to DNA problem
#[derive(Debug, Args)]
#[clap(group(
    clap::ArgGroup::new("input")
        .required(true)
        .args(&["sequence", "seqfile"]),
))]
pub struct DnaArgs {
    /// DNA sequence
    #[clap(short = 's', long)]
    pub sequence: Option<String>,

    /// TXT file with the DNA sequence
    #[clap(short = 'f', long)]
    pub seqfile: Option<PathBuf>,

    /// Filename to be written.
    #[clap(short = 'w', long)]
    pub writefile: Option<PathBuf>,
}
