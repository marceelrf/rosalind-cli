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
    /// Rabbits and Recurrence Relations
    FIB(problems::fib::args::FibArgs),
    /// Mendel's First Law
    IPRB(problems::iprb::args::IprbArgs),
    /// Calculating Expected Offspring
    IEV(problems::iev::args::IevArgs),
    /// Mortal Fibonacci Rabbits
    FIBD(problems::fibd::args::FibdArgs),
    /// Inferring mRNA from Protein
    MRNA(problems::mrna::args::MrnaArgs),
    /// Independent Alleles
    LIA(problems::lia::args::LiaArgs),
    /// Calculating Protein Mass
    PRTM(problems::prtm::args::PrtmArgs),
    /// Finding a Protein Motif
    MPRT(problems::mprt::args::MprtArgs),
    /// RNA Splicing
    SPLC(problems::splc::args::SplcArgs),
    /// Consensus and Profile
    CONS(problems::cons::args::ConsArgs),
    /// Finding a Shared Motif 
    LCSM(problems::lcsm::args::LcsmArgs),

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
        },
        Commands::FIB(args) => {
            problems::fib::solve::solve(&args);
        },
        Commands::IPRB(args) => {
            problems::iprb::solve::solve(&args);
        },
        Commands::IEV(args) => {
            problems::iev::solve::solve(&args);
        },
        Commands::FIBD(args) => {
            problems::fibd::solve::solve(&args);
        },
        Commands::MRNA(args) =>{
            problems::mrna::solve::solve(&args);
        },
        Commands::LIA(args) => {
            problems::lia::solve::solve(&args);
        },
        Commands::PRTM(args) => {
            problems::prtm::solve::solve(&args);
        },
        Commands::MPRT(args) => {
            problems::mprt::solve::solve(&args);
        },
        Commands::SPLC(args) => {
            problems::splc::solve::solve(&args);
        },
        Commands::CONS(args) => {
            problems::cons::solve::solve(&args);
        },
         Commands::LCSM(args) => {
            problems::lcsm::solve::solve(&args);
        }
        // Next commands
    }
}