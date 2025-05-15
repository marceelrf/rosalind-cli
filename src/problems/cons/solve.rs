use super::args::ConsArgs;
use crate::utils::sequence::{read_fasta, count_bases_by_position, consensus_sequence};
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn solve(args: &ConsArgs) {

    // Read the path to fasta file
    let path = args
            .readfile
            .as_ref().expect("Mandatory input file")
            .to_str()
            .expect("Error converting path to string");

    let seqs = read_fasta(path);

    if let Some((&_, first_seq)) = seqs.iter().next() {
        let expected_len = first_seq.len();
        
        // Checks that all other sequences are the same length
        for (id, seq) in &seqs {
            if seq.len() != expected_len {
                panic!("The sequence with ID '{}' has a different length {} than expected ({}).", id, seq.len(), expected_len);
            }
        }
    } else {
        panic!("The FASTA file is empty!");
    }

    let counts = count_bases_by_position(&seqs);

    let consensus = consensus_sequence(&counts);

    let mut output = String::new();
    output.push_str(&format!("{}\n", consensus));

    let bases = ['A', 'C', 'G', 'T'];
    for base in &bases {
        output.push_str(&format!(
            "{}: {}\n",
            base,
            counts
                .iter()
                .map(|m| m.get(base).unwrap_or(&0).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }

    println!("{}", output);

    if let Some(path) = &args.writefile {
        let file = File::create(path).expect("Error creating output file");
        let mut writer = BufWriter::new(file);
        writer.write_all(output.as_bytes()).expect("Error writing to the file");
    }



}
