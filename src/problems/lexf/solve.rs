use super::args::LexfArgs;
use crate::utils::sequence::read_lines;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

pub fn solve(args: &LexfArgs) {

    let (letters, n) = if let Some(file) = args.readfile.as_ref() {

        let lines = read_lines(file.to_str().expect("Invalid path"));

        if lines.len() != 2 {
            panic!("File must has two lines!");
        }

         let words: Vec<String> = lines[0]
         .split_whitespace()
         .map(|s| s.to_string())
         .collect();
        
        let numero: i32 = lines[1]
        .trim()
        .parse()
        .expect("The second line must contain an integer.");

        (words, numero)
    } else {
        panic!("No input file provided.");
    };

    let results: Vec<String> = std::iter::repeat(letters.clone())
    .take(n as usize)
    .multi_cartesian_product()
    .map(|combo| combo.join(""))
    .collect();

    if args.printresult {
        for r in &results {
            println!("{}", r);
        }
    }

    // Save to file if specified
    if let Some(write_path) = &args.writefile {
        let mut file = File::create(write_path)
            .expect("Failed to create output file");
        let output = results.join("\n");

        file.write_all(output.as_bytes())
            .expect("Failed to write to file");
    }

}