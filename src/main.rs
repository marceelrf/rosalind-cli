use clap::{Parser, Subcommand};


mod problems;
pub mod utils;

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
    arg_required_else_help = true,  // Displays help if no argument is passed
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Counting DNA Nucleotides
    DNA(problems::dna::args::DnaArgs),
    /// Transcribing DNA into RNA
    RNA(problems::rna::args::RnaArgs),
    /// Complementing a Strand of DNA
    REVC(problems::revc::args::RevcArgs),
    /// Computing GC Content
    GC(problems::gc::args::GcArgs),
    /// Translating RNA into Protein
    PROT(problems::prot::args::ProtArgs),
    /// Finding a Motif in DNA
    SUBS(problems::subs::args::SubsArgs),
    /// Counting Point Mutations
    HAMM(problems::hamm::args::HammArgs),

    // Here you can then add more problems:
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
        },
        Commands::GC(args) => {
            problems::gc::solve::solve(&args);
        },
        Commands::PROT(args) => {
            problems::prot::solve::solve(&args);
        },
        Commands::SUBS(args) => {
            problems::subs::solve::solve(&args);
        },
        Commands::HAMM(args) => {
            problems::hamm::solve::solve(&args);
        }
        // Next commands
    }
}