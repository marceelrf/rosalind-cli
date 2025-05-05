use super::args::MrnaArgs;
use std::fs;
use crate::utils::genetic_code::rna_codons;
use crate::utils::sequence::read_lines;
use std::collections::HashMap;

pub fn solve(args: &MrnaArgs) {

    let path_str = args
    .readfile
    .as_ref()                 // Option<&PathBuf>
    .expect("Mandatory input file")
    .to_str()                 // Option<&str>
    .expect("Error converting path to string");

    let protein: String = read_lines(path_str).remove(0);
    let codons = rna_codons();
    let modulo = 1_000_000;

    let mut counts: usize = 1;

    for aa in protein.chars() {

        let num_codons = count_codons(&codons, aa);
        counts = (counts * num_codons) % modulo;
    }

   // Include the STOP codons
   counts = (counts * 3) % modulo;

   // Print the output
   println!("{}", counts);

    // Export the Protein sequence
    if let Some(file) = &args.writefile {
        fs::write(file, counts.to_string()).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }

}

fn count_codons(codon_table: &HashMap<&str, &str>, aa: char) -> usize {
    
    codon_table.values().filter(|&&v| v.chars().next().unwrap() == aa).count()

}