pub mod dna;
pub mod rna;
// Adicione novos problemas aqui

use clap::Subcommand;

/// Enum de todos os problemas disponíveis
#[derive(Subcommand)]
pub enum ProblemCommands {
    #[command(subcommand)]
    Dna(dna::DnaCommand),
    
    // Padrão repetido para outros problemas:
    // Rna(rna::RnaCommand),
    // Revc(revc::RevcCommand),
}