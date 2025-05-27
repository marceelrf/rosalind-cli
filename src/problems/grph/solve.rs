use super::args::GrphArgs;
use crate::utils::sequence::{read_fasta,has_overlap};
use std::fs::File;
use std::io::Write;


pub fn solve(args: &GrphArgs){

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
    
    let k = args.overlapsize;

    let mut result = String::new();
        
    for (id1, seq1) in &seqs {
        for (id2, seq2) in &seqs {
            if id1 != id2 && has_overlap(seq1, seq2, k) {
                 result.push_str(&format!("{} {}\n", id1, id2));
            }
        }
    }

     // Print to stdout if enabled
    if args.printresult {
        print!("{}", result);
    }

    // Save to file if provided
    if let Some(write_path) = &args.writefile {
        let mut file = File::create(write_path)
            .expect("Failed to create output file");
        file.write_all(result.as_bytes())
            .expect("Failed to write to file");
    }


}