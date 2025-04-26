use std::path::PathBuf;
use clap::Args;

/// Argumentos para resolver o problema DNA
#[derive(Args)]
pub struct DnaArgs {
    #[arg(short, long, conflicts_with = "seqfile")]
    pub sequence: Option<String>,

    #[arg(short, long)]
    pub seqfile: Option<PathBuf>,

    #[arg(short, long)]
    pub writefile: Option<PathBuf>,
}
