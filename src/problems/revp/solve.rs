use super::args::RevpArgs;
use crate::utils::sequence::{read_fasta, find_reverse_palindromes};
use std::fs::File;
use std::io::Write;

pub fn solve(args: &RevpArgs){

    let path_str = args
    .readfile
    .as_ref()                 // Option<&PathBuf>
    .expect("Mandatory input file")
    .to_str()                 // Option<&str>
    .expect("Error converting path to string");

    let seqs = read_fasta(path_str);

    if seqs.len() != 1 {
        panic!("Expected exactly one sequence in the FASTA file, but found {}.", seqs.len());
    }

    let seq = seqs.values().next().unwrap();

    let results = find_reverse_palindromes(seq);

    let mut result = String::new();
    for (pos, len) in results {
        result.push_str(&format!("{} {}\n", pos, len));
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