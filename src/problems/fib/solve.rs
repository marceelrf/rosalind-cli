use super::args::FibArgs;
use crate::utils::sequence::read_lines;
use std::fs;

pub fn solve(args: &FibArgs) {
    // Read the values of n and k from the file or the command line
    let (n, k) = if let Some(file) = &args.readfile {
        let lines = read_lines(file.to_str().expect("Invalid path"));

        let values: Vec<u32> = lines[0]
        .trim()
        .split_whitespace()
        .map(|v| v.parse().expect("Could not parse integer"))
        .collect();

        if values.len() != 2 {
            panic!("Expected two integers (n and k) in the file.");
        }
        
        //let (n, k) = (values[0], values[1]);
        (values[0], values[1])
    } else {
        // unwrap() safe here, as the “direct” group requires both to be present
        (args.n.unwrap(), args.k.unwrap())
    };

    // Calculate the number of pairs of rabbits
    let result = fib(n, k);

    // Print on screen
    println!("{}", result);

    // Write to file, if provided
    if let Some(file) = &args.writefile {
        fs::write(file, result.to_string()).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }
}

fn fib(n: u32, k: u32) -> usize {
    let mut prev = 1;
    let mut curr = 1;

    for _ in 2..n {
        let next = curr + prev * k as usize;
        prev = curr;
        curr = next;
    }

    curr
}
