use colored::*;
use std::collections::HashMap;

/// Returns a HashMap with the standard genetic code (RNA → aminoacid)
pub fn rna_codons() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("UUU", "F"), ("UUC", "F"), ("UUA", "L"), ("UUG", "L"),
        ("UCU", "S"), ("UCC", "S"), ("UCA", "S"), ("UCG", "S"),
        ("UAU", "Y"), ("UAC", "Y"), ("UAA", "*"), ("UAG", "*"),
        ("UGU", "C"), ("UGC", "C"), ("UGA", "*"), ("UGG", "W"),
        ("CUU", "L"), ("CUC", "L"), ("CUA", "L"), ("CUG", "L"),
        ("CCU", "P"), ("CCC", "P"), ("CCA", "P"), ("CCG", "P"),
        ("CAU", "H"), ("CAC", "H"), ("CAA", "Q"), ("CAG", "Q"),
        ("CGU", "R"), ("CGC", "R"), ("CGA", "R"), ("CGG", "R"),
        ("AUU", "I"), ("AUC", "I"), ("AUA", "I"), ("AUG", "M"),
        ("ACU", "T"), ("ACC", "T"), ("ACA", "T"), ("ACG", "T"),
        ("AAU", "N"), ("AAC", "N"), ("AAA", "K"), ("AAG", "K"),
        ("AGU", "S"), ("AGC", "S"), ("AGA", "R"), ("AGG", "R"),
        ("GUU", "V"), ("GUC", "V"), ("GUA", "V"), ("GUG", "V"),
        ("GCU", "A"), ("GCC", "A"), ("GCA", "A"), ("GCG", "A"),
        ("GAU", "D"), ("GAC", "D"), ("GAA", "E"), ("GAG", "E"),
        ("GGU", "G"), ("GGC", "G"), ("GGA", "G"), ("GGG", "G"),
    ])
}

/// Returns a HashMap with the standard genetic code (DNA → amino acid)
pub fn dna_codons() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("TTT", "F"), ("TTC", "F"), ("TTA", "L"), ("TTG", "L"),
        ("TCT", "S"), ("TCC", "S"), ("TCA", "S"), ("TCG", "S"),
        ("TAT", "Y"), ("TAC", "Y"), ("TAA", "*"), ("TAG", "*"),
        ("TGT", "C"), ("TGC", "C"), ("TGA", "*"), ("TGG", "W"),
        ("CTT", "L"), ("CTC", "L"), ("CTA", "L"), ("CTG", "L"),
        ("CCT", "P"), ("CCC", "P"), ("CCA", "P"), ("CCG", "P"),
        ("CAT", "H"), ("CAC", "H"), ("CAA", "Q"), ("CAG", "Q"),
        ("CGT", "R"), ("CGC", "R"), ("CGA", "R"), ("CGG", "R"),
        ("ATT", "I"), ("ATC", "I"), ("ATA", "I"), ("ATG", "M"),
        ("ACT", "T"), ("ACC", "T"), ("ACA", "T"), ("ACG", "T"),
        ("AAT", "N"), ("AAC", "N"), ("AAA", "K"), ("AAG", "K"),
        ("AGT", "S"), ("AGC", "S"), ("AGA", "R"), ("AGG", "R"),
        ("GTT", "V"), ("GTC", "V"), ("GTA", "V"), ("GTG", "V"),
        ("GCT", "A"), ("GCC", "A"), ("GCA", "A"), ("GCG", "A"),
        ("GAT", "D"), ("GAC", "D"), ("GAA", "E"), ("GAG", "E"),
        ("GGT", "G"), ("GGC", "G"), ("GGA", "G"), ("GGG", "G"),
    ])
}
/// Translates a DNA sequence into an amino acid chain
pub fn translate(seq: &str, codon_table: &HashMap<&str, &str>) -> String {
    let mut protein = String::new();

    for codon in seq
        .as_bytes()
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or("")) 
        .filter(|c| c.len() == 3)
    {
        match codon_table.get(codon) {
            Some(&aa) if aa != "*" => protein.push_str(aa),
            Some(_) => break, // If found a stop codon
            None => continue, // invalid/incomplete codon
        }
    }

    protein
}

/// Prints the protein sequence with coloring based on the polarity of the side chain
pub fn print_colored_protein(protein: &str) {
    for aa in protein.chars() {
        let colored_aa = match aa {
            // Non-polar
            'A' | 'V' | 'L' | 'I' | 'M' | 'F' | 'W' | 'P' | 'G' => aa.to_string().yellow(),
            // Polars without charge
            'S' | 'T' | 'C' | 'Y' | 'N' | 'Q' => aa.to_string().blue(),
            // Positively charged polar
            'K' | 'R' | 'H' => aa.to_string().green(),
            // Negatively charged polar
            'D' | 'E' => aa.to_string().red(),
            // Other (or stop)
            _ => aa.to_string().white(),
        };
        print!("{}", colored_aa);
    }
    println!();
}
/// Find ORFs from a DNA sequence
pub fn find_orfs(seq: &str, codon_table: &HashMap<&str, &str>) -> Vec<String> {
    let mut proteins = Vec::new();

    for frame in 0..3 {
        let chars: Vec<char> = seq.chars().collect();
        let mut i = frame;
        while i + 3 <= chars.len() {
            let codon: String = chars[i..i + 3].iter().collect();
            if codon_table.get(&codon[..]).copied() == Some("M") {
                let mut protein = String::new();
                let mut j = i;
                while j + 3 <= chars.len() {
                    let codon: String = chars[j..j + 3].iter().collect();
                    let aa = codon_table.get(&codon[..]).copied().unwrap_or("X");
                    if aa == "*" {
                        protein.push_str("*");
                        proteins.push(protein.trim_end_matches('*').to_string());
                        break;
                    } else {
                        protein.push_str(aa);
                    }
                    j += 3;
                }
            }
            i += 3;
        }
    }

    proteins
}
