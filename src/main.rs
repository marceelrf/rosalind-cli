use clap::{Parser, Subcommand};

mod problems;  // Importa o módulo problems/

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Problem,
}

#[derive(Subcommand)]
enum Problem {
    /// Resolve o problema 'Counting DNA Nucleotides' (DNA)
    DNA {
        /// Sequência de DNA de entrada
        #[arg(short, long)]
        dna: String,
    },
    RNA {
        #[arg(short, long)]
        dna: String,
    }
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Problem::DNA { dna } => {
            problems::dna::solve(&dna);
        },
        Problem::RNA { dna} => {
            problems::rna::solve(&dna);
        },
    }
}
