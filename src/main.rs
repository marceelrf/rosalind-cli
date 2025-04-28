use clap::{Parser, Subcommand};

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
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve the DNA problem.
    DNA(problems::dna::args::DnaArgs),
    /// Solve the RNA problem.
    RNA(problems::rna::args::RnaArgs),
    /// Sove the REVC problem.
    REVC(problems::revc::args::RevcArgs)

    // Aqui depois você pode adicionar mais problemas:
    // Prob2(problems::prob2::args::Prob2Args),
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::DNA(args) => {
            problems::dna::solve::solve(&args);
        },
        Commands::RNA(args) => {
            problems::rna::solve::solve(&args);
        },
        Commands::REVC(args) => {
            problems::revc::solve::solve(&args);
        }
        // Next commands
    }
}