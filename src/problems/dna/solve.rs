use super::args::DnaArgs;
use colored::*;
use std::fs;

/// Conta e imprime nucleotídeos com cores no estilo IGV
pub fn solve(args: &DnaArgs) {
    // Ler a sequência (do argumento ou arquivo)
    let dna: String = if let Some(seq) = &args.dna {
        seq.to_string()
    } else if let Some(file) = &args.seqfile {
        fs::read_to_string(file).unwrap_or_else(|_| {
            eprintln!("Error: The file could not be read {}", file.display());
            std::process::exit(1);
        })
    } else {
        unreachable!() // Garantido por ArgGroup
    }.trim().to_string();

    // Validar sequência
    if !dna.chars().all(|c| matches!(c, 'A' | 'T' | 'C' | 'G' | 'a' | 't' | 'c' | 'g')) {
        eprintln!("Error: String contains invalid characters");
        std::process::exit(1);
    }

    let dna = dna.to_uppercase();
    let counts = (
        dna.chars().filter(|&c| c == 'A').count(),
        dna.chars().filter(|&c| c == 'C').count(),
        dna.chars().filter(|&c| c == 'G').count(),
        dna.chars().filter(|&c| c == 'T').count(),
    );

    // Imprimir sequência colorida
    for c in dna.chars() {
        let colored_base = match c {
            'A' => "A".bright_green(),
            'T' => "T".bright_red(),
            'C' => "C".bright_blue(),
            'G' => "G".bright_yellow(),
            _ => c.to_string().white(),
        };
        print!("{}", colored_base);
    }
    println!();

    // Imprimir contagem colorida
    println!(
        "{}: {}, {}: {}, {}: {}, {}: {}",
        "A".bright_green(), counts.0,
        "C".bright_blue(), counts.1,
        "G".bright_yellow(), counts.2,
        "T".bright_red(), counts.3
    );

    // Salvar em arquivo se --writefile for especificado
    if let Some(file) = &args.writefile {
        let output = format!("{} {} {} {}", counts.0, counts.1, counts.2, counts.3);
        fs::write(file, output).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }
}