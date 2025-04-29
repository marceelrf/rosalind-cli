//use std::fmt::format;
use std::fs;
use super::args::GcArgs;
use colored::*;
use crate::utils::sequence::{read_fasta, get_highest_gc};
//use std::process;

pub fn solve(args: &GcArgs) {
    // args = fasta & writefile
    // 
    let fasta_path: &str = args.fasta.to_str().expect("Invalid UTF-8 in file path");


    let records = read_fasta(fasta_path);

    if let Some((id, gc)) = get_highest_gc(&records) {
        println!("{}\n{:.4}%", id.red(), gc);
    
        // Save to file if --writefile is specified
        if let Some(file) = &args.writefile {
            let output = format!("{}\n{:.4}", id, gc);
            fs::write(file, output).unwrap_or_else(|_| {
                eprintln!("Error: Could not write to file {}", file.display());
            });
        }
    } else {
        println!("No sequences found.");
    }

}