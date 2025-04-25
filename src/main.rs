use clap::{Parser, Subcommand};

mod problems;  // Importa o módulo problems/

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
    command: Option<Problem>,
}

#[derive(Subcommand)]
enum Problem {
    /// Solve the problem ‘Counting DNA Nucleotides’ (DNA)
    #[command(arg_required_else_help = true)]
    DNA {
        /// DNA sequence <string>
        #[arg(short, long)]
        dna: String,
    },
    /// Transcribes DNA into RNA (RNA problem)
    #[command(arg_required_else_help = true)]
    RNA {
        /// DNA sequence <string>
        #[arg(short, long)]
        dna: String,
    }
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Problem::DNA { dna }) => {
            problems::dna::solve(&dna);
        },
        Some(Problem::RNA { dna}) => {
            problems::rna::solve(&dna);
        },
        None => {

        }
    }
}
