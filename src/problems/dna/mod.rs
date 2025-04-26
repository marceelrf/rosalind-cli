pub mod args;
pub mod solve;

use clap::Args;
use clap::ArgGroup;
use super::super::ProblemCommands;

/// Configuração do comando DNA
#[derive(Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["sequence", "seqfile", "writefile"]),
))]
#[derive(Args)]
pub struct DnaCommand {
    #[command(flatten)]
    pub args: crate::problems::dna::args::DnaArgs,
}

impl From<DnaCommand> for ProblemCommands {
    fn from(cmd: DnaCommand) -> Self {
        ProblemCommands::Dna(cmd)
    }
}