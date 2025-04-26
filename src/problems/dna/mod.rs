pub mod args;
pub mod solve;

use clap::Args;

/// Configuração do comando DNA
#[derive(Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["sequence", "seqfile"]),
))]
pub struct DnaCommand {
    #[command(flatten)]
    pub args: args::DnaArgs,
}