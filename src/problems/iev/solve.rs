use std::fs;
use super::args::IevArgs;
use crate::utils::sequence::read_lines;

pub fn solve(args: &IevArgs){
     // Args: -f, -w

    // Read values from command line
    let (v, w, x, y, z, _) = if let Some(file) = &args.readfile {
        let lines = read_lines(file.to_str().expect("Invalid path"));
        let values: Vec<u32> = lines[0]
            .trim()
            .split_whitespace()
            .map(|v| v.parse().expect("Could not parse integer"))
            .collect();
    
        if values.len() != 6 {
            panic!("Expected six integers (v, w, x, y, z, aa) in the file.");
        }
    
        (values[0], values[1], values[2], values[3], values[4], values[5])
    } else {
        panic!("Missing input file");
    };

    // The result of Expected offspring with dominant alleles.
    let result = expected_offspring(v, w, x, y, z);

    // Print the result
    println!("{:.4}", result);

     // Write to file, if provided
     if let Some(file) = &args.writefile {
        fs::write(file, result.to_string()).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }


}

fn expected_offspring(v: u32, w: u32, x: u32, y: u32, z: u32) -> f64 {
    let v = v as f64;
    let w = w as f64;
    let x = x as f64;
    let y = y as f64;
    let z = z as f64;

    2.0 * (v + w + x) + 1.5 * y + z
}
