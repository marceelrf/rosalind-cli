use super::args::ProtArgs;
//use colored::*;
use std::fs;
use crate::utils::genetic_code::{rna_codons,translate, print_colored_protein};
use crate::utils::sequence::{read_sequence, validate_rna};

/// Solve the PROT problem
pub fn solve(args: &ProtArgs){
    // Import the RNA sequence
    let rna: String = read_sequence(args.sequence.as_ref(), args.seqfile.as_ref());

    // Validate the RNA sequence
    validate_rna(&rna);

    let rna = rna.to_uppercase();

    // Start the translation
    let codons = rna_codons();

    let protein = translate(&rna, &codons);

    print_colored_protein(&protein);

    // Export the Protein sequence
    if let Some(file) = &args.writefile {
        fs::write(file, protein).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }



}