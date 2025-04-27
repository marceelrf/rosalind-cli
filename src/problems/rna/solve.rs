use super::args::RnaArgs;
use colored::*;
use std::fs;

/// Transcribe a DNA sequence into RNA by coloring the bases
pub fn solve(args: &RnaArgs) {
    // Ler a sequência (do argumento ou arquivo)
    let dna: String = if let Some(seq) = &args.sequence {
        seq.to_string()
    } else if let Some(file) = &args.seqfile {
        fs::read_to_string(file).unwrap_or_else(|_| {
            eprintln!("Error: The file could not be read {}", file.display());
            std::process::exit(1);
        })
    } else {
        unreachable!() // Garantido por ArgGroup
    }.trim().to_string();

    // Validate sequence
    if !dna.chars().all(|c| matches!(c, 'A' | 'T' | 'C' | 'G' | 'a' | 't' | 'c' | 'g')) {
        eprintln!("Error: String contains invalid characters");
        std::process::exit(1);
    }

    // Transcrition: replace T by U
    let rna = dna
        .to_uppercase()
        .chars()
        .map(|c| if c == 'T' { 'U' } else { c })
        .collect::<String>();

    // Print the  colored RNA
    for c in rna.chars() {
        let colored_base = match c {
            'A' => "A".bright_green(),
            'U' => "U".bright_red(),
            'C' => "C".bright_blue(),
            'G' => "G".bright_yellow(),
            _ => c.to_string().white(),
        };
        print!("{}", colored_base);
    }
    println!();

    // Save to file if --writefile is specified
    if let Some(file) = &args.writefile {
        fs::write(file, rna).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }
}