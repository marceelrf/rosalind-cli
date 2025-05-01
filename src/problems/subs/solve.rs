use super::args::SubsArgs;
use crate::utils::sequence::{read_lines, find_substring_matches};
use std::fs;


pub fn solve(args: &SubsArgs){
    // Args: readfile and writefile
    let file_path: &str = args.readfile.to_str().expect("Invalid UTF-8 in file path");
    
    let lines = read_lines(file_path);
    let sequence = &lines[0];
    let pattern = &lines[1];

    let position = find_substring_matches(sequence, pattern);

    let output = position
    .iter()
    .map(|pos| pos.to_string())
    .collect::<Vec<_>>()
    .join(" ");

    println!("{}", output);

    if let Some(file) = &args.writefile {
        fs::write(file, output).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }

}