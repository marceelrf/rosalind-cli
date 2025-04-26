use clap::Parser;
use problems::ProblemCommands;

mod problems;

#[derive(Parser)]
#[command(
    name = "rosalind",
    author = "Marcel Ferreira",
    version = "0.1.0",
    about = "🔬 Rosalind troubleshooting tool (Bioinformatics)",
    long_about = r#"
    Examples:
      rosalind-cli dna --sequence "AGCTTTTCATT"
      rosalind-cli rna --sequence "GATGGAACTTGACTACGTAA"

    https://github.com/marceelrf/rosalind-cli
    "#,
    arg_required_else_help = true,  // Exibe ajuda se nenhum argumento for passado
)]
struct Cli {
    #[command(subcommand)]
    command: ProblemCommands,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        ProblemCommands::Dna(cmd) => problems::dna::solve(&cmd.args),
        // ProblemCommands::Rna(cmd) => problems::rna::solve(&cmd.args),
    }
}