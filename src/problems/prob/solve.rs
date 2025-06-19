use crate::utils::sequence::read_lines;
use super::args::ProbArgs;
use std::process;
use std::fs::File;
use std::io::Write;

pub fn solve(args: &ProbArgs) {

    let file_path: &str = args.readfile
        .as_ref()
        .expect("Missing input file paht")
        .to_str().expect("Invalid UTF-8 in file path");

    let lines = read_lines(file_path);

     if lines.len() != 2 {
        eprintln!(
            "Error: You must provide a two-line file. 
             First line: a DNA sequence.
             Second line: a list of GC probabilities separated by spaces."
        );
        process::exit(1);
    }

    let dna = lines[0].trim();
    let probs_line = lines[1].trim();

    let gc_probs: Vec<f64> = probs_line
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect("Invalid number in GC probs line"))
        .collect();

    let mut result_vec = Vec::new();

    for &gc in &gc_probs {
        let p_gc = gc / 2.0;
        let p_at = (1.0 - gc) / 2.0;

        let mut log_prob = 0.0;

        for base in dna.chars() {
            match base {
                'G' | 'C' => log_prob += p_gc.log10(),
                'A' | 'T' => log_prob += p_at.log10(),
                _ => {
                    eprintln!("Invalid base found in DNA: {}", base);
                    process::exit(1);
                }
            }
        }

        result_vec.push(format!("{:.3}", log_prob));
    }

    let result = result_vec.join(" ");
    if args.printresult {
        print!("{}\n", result);
    }

    if let Some(write_path) = &args.writefile {
        let mut file = File::create(write_path)
            .expect("Failed to create output file");
        file.write_all(result.to_string().as_bytes())
            .expect("Failed to write to file");
    }

}