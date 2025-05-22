// lcsm - solve.rs
use super::args::LcsmArgs;
use crate::utils::sequence::read_fasta;
use std::fs::File;
use std::io::Write;

pub fn solve(args: &LcsmArgs) {

    // Read the path to fasta file
    let path = args
            .readfile
            .as_ref().expect("Mandatory input file")
            .to_str()
            .expect("Error converting path to string");

    let seqs = read_fasta(path);

    if seqs.is_empty() {
        panic!("No sequences found in the FASTA file.");
    }

    let min_seq = seqs
        .values()
        .min_by_key(|seq| seq.len())
        .expect("Failed to find the shortest sequence");

    let mut result = String::new();

    // Iterate over the possbiles sequences
    'outer: for len in (1..=min_seq.len()).rev() {
        for i in 0..=min_seq.len() - len {
            let candidate = &min_seq[i..i + len];

            if seqs.values().all(|seq| seq.contains(candidate)) {
                result = candidate.to_string();
                break 'outer;
            }
        }
    }

    println!("{}", result);

    if let Some(write_path) = &args.writefile {
        let mut file = File::create(write_path)
            .expect("Failed to create output file");
        writeln!(file, "{}", result)
            .expect("Failed to write to file");
    }

}