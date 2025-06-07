use std::fs;
//use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// Import a DNA/RNA sequence from a file
pub fn read_sequence(source: Option<&String>, file: Option<&std::path::PathBuf>) -> String {
    if let Some(seq) = source {
        seq.trim().to_string()
    } else if let Some(file) = file {
        fs::read_to_string(file).unwrap_or_else(|_| {
            eprintln!("Error: Could not read file {}", file.display());
            std::process::exit(1);
        }).trim().to_string()
    } else {
        unreachable!()
    }
}
/// Check if a DNA sequence is valid
pub fn validate_dna(seq: &str) {
    if !seq.chars().all(|c| matches!(c, 'A' | 'T' | 'C' | 'G' | 'a' | 't' | 'c' | 'g')) {
        eprintln!("Error: Invalid DNA characters detected.");
        std::process::exit(1);
    }
}
/// Check if a RNA sequence is valid
pub fn validate_rna(seq: &str) {
    if !seq.chars().all(|c| matches!(c, 'A' | 'U' | 'C' | 'G' | 'a' | 'u' | 'c' | 'g')) {
        eprintln!("Error: Invalid RNA characters detected.");
        std::process::exit(1);
    }
}
/// Import the two line file withe the DNA and the pattern
pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Could not open file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}
/// Import a multifasta file into a HashMap
pub fn read_fasta(path: &str) -> HashMap<String, String> {
    let contents = fs::read_to_string(path)
        .expect("Could not read FASTA file");

    let mut records = HashMap::new();
    let mut current_id = String::new();
    let mut current_seq = String::new();

    for line in contents.lines() {
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.insert(current_id.clone(), current_seq.clone());
                current_seq.clear();
            }
            current_id = line[1..].trim().to_string(); // remove '>'
        } else {
            current_seq.push_str(line.trim());
        }
    }

    if !current_id.is_empty() {
        records.insert(current_id, current_seq);
    }

    records
}
/// Compute the GC percent of a DNA string
pub fn gc_percent(dna: &str) -> f64 {
    // Get the sequence lenght
    let seq_len = dna.len() as f64;
    // Count the GC content
    let gc_count = dna.chars().filter(|c| *c == 'G' || *c == 'C').count() as f64;
    // Compute the GC percent
    let gc_percent = 100. * gc_count/seq_len;

    gc_percent
}
/// Get the highest GC sequence
pub fn get_highest_gc(records: &HashMap<String, String>) -> Option<(String, f64)> {
    records
        .iter()
        .map(|(id, seq)| (id.clone(), gc_percent(seq)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) // compare floats
}
/// Find the positions of the substrings
pub fn find_substring_matches(sequence: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let seq_len = sequence.len();
    let pat_len = pattern.len();

    if pat_len > seq_len {
        return positions;
    }

    for i in 0..=(seq_len - pat_len) {
        if &sequence[i..i + pat_len] == pattern {
            positions.push(i + 1); // 1-based index
        }
    }

    positions
}
/// Compute the hamming distance
pub fn hamming_distance(seq1: &str, seq2: &str) -> Result<usize, &'static str> {
    
    if seq1.len() != seq2.len() {
        return Err("Sequence must have the same length.")
    }

    let dist = seq1.chars()
    .zip(seq2.chars())
    .filter(|(a, b)| a != b)
    .count();

    Ok(dist)

}
/// Count the bases by position in a multi-fasta set
pub fn count_bases_by_position(seqs: &HashMap<String, String>) -> Vec<HashMap<char, usize>> {
    if seqs.is_empty() {
        panic!("Nenhuma sequência fornecida.");
    }

    let seq_len = seqs.values().next().unwrap().len();

    let mut position_counts = vec![HashMap::new(); seq_len];

    for seq in seqs.values() {
        for (i, base) in seq.chars().enumerate() {
            *position_counts[i].entry(base).or_insert(0) += 1;
        }
    }

    position_counts
}
/// Get the consensus sequence
pub fn consensus_sequence(counts: &[HashMap<char, usize>]) -> String {
    counts
        .iter()
        .map(|count_map| {
            count_map
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(&base, _)| base)
                .unwrap_or('N')
        })
        .collect()
}
/// Checks if the suffix of one string matches the prefix of another, given an overlap size of k.
pub fn has_overlap(s1: &str, s2: &str, k: usize) -> bool {
    s1.ends_with(&s2[..k])
}
/// Returns the reverse complement of a DNA sequence
pub fn reverse_complement(dna: &str) -> String {
    
    validate_dna(dna);
    
    dna.chars()
        .rev()
        .map(|base| match base {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => base, // ou panic!("Invalid DNA base: {}", base)
        })
        .collect()
}
/// Find the reverse palindromes (From 4 to 12 bases).
pub fn find_reverse_palindromes(seq: &str) -> Vec<(usize, usize)> {
    let mut results = Vec::new();

    for i in 0..seq.len() {
        for len in 4..=12 {
            if i + len <= seq.len() {
                let subseq = &seq[i..i + len];
                if subseq == reverse_complement(subseq) {
                    results.push((i + 1, len)); // posição 1-based
                }
            }
        }
    }

    results
}
/// Count the RNA bases.
pub fn count_rna_bases(seq: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for base in seq.chars() {
        match base {
            'A' | 'U' | 'C' | 'G' => {
                *counts.entry(base).or_insert(0) += 1;
            }
            _ => panic!("Invalid RNA base: {}", base),
        }
    }

    counts
}