use super::args::FibdArgs;
use crate::utils::sequence::read_lines;
use std::fs;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

pub fn solve(args: &FibdArgs) {
    // Read the values of n and m from the file or the command line
    let (n, m) = if let Some(file) = &args.readfile {
        let lines = read_lines(file.to_str().expect("Invalid path"));

        let values: Vec<u64> = lines[0]
            .trim()
            .split_whitespace()
            .map(|v| v.parse().expect("Could not parse integer"))
            .collect();

        if values.len() != 2 {
            panic!("Expected two integers (n and m) in the file.");
        }

        (values[0], values[1])
    } else {
        // unwrap() safe here, as the “direct” group requires both to be present
        (args.n.unwrap() as u64, args.m.unwrap() as u64)
    };

    let results: BigUint = get_pop(n, m as usize);
    
    // Print on screen
    println!("{}", results);

    // Write to file, if provided
    if let Some(file) = &args.writefile {
        fs::write(file, results.to_string()).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }
}

fn get_pop(n: u64, m: usize) -> BigUint {
    let mut ages = vec![BigUint::zero(); m];
    ages[0] = BigUint::one();

    for _ in 1..n {
        let newborns: BigUint = ages[1..].iter().cloned().sum();
        for i in (1..m).rev() {
            ages[i] = replace(&mut ages[i - 1], BigUint::zero());
        }
        ages[0] = newborns;
    }

    ages.iter().cloned().sum()
}