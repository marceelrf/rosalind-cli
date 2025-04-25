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
    DNA(DnaArgs),
    /// Transcribes DNA into RNA (RNA problem)
    #[command(arg_required_else_help = true)]
    RNA {
        /// DNA sequence <string>
        #[arg(short, long)]
        dna: String,
    }
}

pub struct DnaArgs {
    /// DNA sequence (ex: "AGCTTTTCATT")
    #[arg(short, long, conflicts_with = "seqfile")]
    pub dna: Option<String>,

    /// DNA sequence txt file
    #[arg(short, long, value_name = "FILE")]
    pub seqfile: Option<PathBuf>,

    /// Write the count in a txt file (format: "A C G T")
    #[arg(short, long, value_name = "FILE")]
    pub writefile: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Problem::DNA { args }) => {
            problems::dna::solve(&args);
        },
        Some(Problem::RNA { dna}) => {
            problems::rna::solve(&dna);
        },
        None => {

        }
    }
}
