use colored::*;

/// Conta e imprime nucleotídeos com cores no estilo IGV
pub fn solve(dna: &str) {
    let counts = (
        dna.chars().filter(|&c| c == 'A').count(),
        dna.chars().filter(|&c| c == 'C').count(),
        dna.chars().filter(|&c| c == 'G').count(),
        dna.chars().filter(|&c| c == 'T').count(),
    );

    // Imprime cada base com sua cor
    for c in dna.chars() {
        let colored_base = match c {
            'A' => "A".bright_green(),
            'T' => "T".bright_red(),
            'C' => "C".bright_blue(),
            'G' => "G".bright_yellow(),
            _ => c.to_string().white(),  // Caracteres inválidos em branco
        };
        print!("{}", colored_base);
    }
    println!();  // Quebra de linha após a sequência colorida

    // Imprime a contagem (opcional: também colorida)
    println!(
        "A: {}, C: {}, G: {}, T: {}",
        counts.0.to_string().bright_green(),
        counts.1.to_string().bright_blue(),
        counts.2.to_string().bright_yellow(),
        counts.3.to_string().bright_red()
    );
}