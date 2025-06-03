use super::args::OrfArgs;
use crate::utils::sequence::{read_fasta, reverse_complement};
use crate::utils::genetic_code::{dna_codons,find_orfs};
use std::collections::{HashSet};
use std::fs::File;
use std::io::Write;


pub fn solve(args: &OrfArgs) {

    // Read the path to fasta file
    let path = args
            .readfile
            .as_ref().expect("Mandatory input file")
            .to_str()
            .expect("Error converting path to string");

    let seqs = read_fasta(path);

    if seqs.len() != 1 {
        panic!("Expected exactly one sequence in the FASTA file.");
    }

    let seq = seqs.values().next().unwrap();
    let codons = dna_codons();

    let mut all_orfs = HashSet::new();

    // 3 frames direct
    all_orfs.extend(find_orfs(seq, &codons));
    // 3 frames reverse
    all_orfs.extend(find_orfs(&reverse_complement(seq), &codons));

    let mut result = String::new();
    for orf in &all_orfs {
        result.push_str(orf);
        result.push('\n');
    }

    // Print to stdout if enabled
    if args.printresult {
        print!("{}", result);
    }

    // Save to file if specified
    if let Some(write_path) = &args.writefile {
        let mut file = File::create(write_path)
            .expect("Failed to create output file");
        file.write_all(result.as_bytes())
            .expect("Failed to write to file");
    }

}