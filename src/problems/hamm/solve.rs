use super::args::HammArgs;
use crate::utils::sequence::{read_lines, validate_dna, hamming_distance};
use std::fs;
use std::process;

pub fn solve(args: &HammArgs){
    // Args: readfile & writefile

    let file_path: &str = args.readfile.to_str().expect("Invalid UTF-8 in file path");

    let lines = read_lines(file_path);

    if lines.len() != 2 {
        eprintln!("Error: You must provide exactly two DNA sequences, one per line.");
        process::exit(1);
    }

    let (seq1, seq2) = (&lines[0], &lines[1]);

    validate_dna(seq1);
    validate_dna(seq2);

    match hamming_distance(seq1, seq2) {
        Ok(dist) => {
            println!("{}", dist);

            if let Some(file) = &args.writefile {
                if let Err(e) = fs::write(file, dist.to_string()) {
                    eprintln!("Error: Could not write to file {}: {}", file.display(), e);
                }
            }
        }
        Err(e) => {
            eprintln!("Hamming distance error: {}", e);
            process::exit(1);
        }
    }
}