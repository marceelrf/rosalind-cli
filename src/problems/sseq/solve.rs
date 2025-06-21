use super::args::SseqArgs;
use crate::utils::sequence::read_lines;
use std::process;


pub fn solve(args: &SseqArgs){

    let file_path: &str = args.readfile
        .as_ref()
        .expect("Missing input file paht")
        .to_str().expect("Invalid UTF-8 in file path");

    let lines = read_lines(file_path);
    
    if lines.len() != 2 {
        eprintln!(
            "Error: You must provide a two-line file. 
             First line: a DNA sequence.
             Second line: a substring"
        );
        process::exit(1);
    }

    let dna = lines[0].trim();
    let sseq = lines[1].trim();


}