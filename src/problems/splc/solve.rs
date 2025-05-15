use super::args::SplcArgs;
use crate::utils::sequence::read_fasta;
use crate::utils::genetic_code::{dna_codons,translate};
use std::fs;

pub fn solve(args: &SplcArgs){
    // Read the path to fasta file
    let path = args
            .readfile
            .as_ref().expect("Mandatory input file")
            .to_str()
            .expect("Error converting path to string");

    let mut seqs = read_fasta(path);
    // The DNA sequence using seq size
    let longest_id = {
        seqs
        .iter()
        .max_by_key(|(_, seq)| seq.len())
        .map(|(id, _)| id.clone())
        .expect("No sequences found")
    };
    let longest_seq = seqs
                                .remove(&longest_id)
                                .expect("Could not remove the longest sequence");

    

     let mut exon_seq = longest_seq.clone();
     
     for intron_seq in seqs.values() {
        exon_seq = exon_seq.replace(intron_seq, "");
    }
    
    let codons = dna_codons();
    let protein = translate(&exon_seq, &codons);

    println!("{}", protein);

    // Export the Protein sequence
    if let Some(file) = &args.writefile {
        fs::write(file, protein).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }


}