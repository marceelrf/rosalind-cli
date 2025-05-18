// lcsm - solve.rs
use super::args::LcsmArgs;
use crate::utils::sequence::read_fasta;

pub fn solve(args: LcsmArgs) {

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



}