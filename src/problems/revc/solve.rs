use super::args::RevcArgs;
use colored::*;
use std::fs;

pub fn solve(args: &RevcArgs){

    let dna: String = if let Some(seq) = &args.sequence {
        seq.to_string()
    } else if let Some(file) = &args.seqfile {
        fs::read_to_string(file).unwrap_or_else(|_| {
            eprintln!("Error: The file could not be read {}", file.display());
            std::process::exit(1);
        })
    } else {
        unreachable!()
    }.trim().to_string();

    if !dna.chars().all(|c| matches!(c, 'A' | 'T' | 'C' | 'G' | 'a' | 't' | 'c' | 'g')) {
        eprintln!("Error: String contains invalid characters");
        std::process::exit(1);
    }

    let dna = dna.to_uppercase();
    let revc_dna: String = dna.chars().rev().map(|c| match c {
        'A' => 'T', 'T' => 'A',
        'C' => 'G', 'G' => 'C',
        _ => c
    }).collect();

    for c in revc_dna.chars() {
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

    // Save to file if --writefile is specified
    if let Some(file) = &args.writefile {
        fs::write(file, revc_dna).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }


}
