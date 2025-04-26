pub mod dna;
pub mod rna; 
// Adicione novos problemas aqui no futuro

use clap::Subcommand;

/// Enum de todos os problemas disponíveis
#[derive(Subcommand)]
pub enum ProblemCommands {
    Dna(dna::DnaCommand),
    // Rna(rna::RnaCommand),
    // Outros problemas futuros
}
