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