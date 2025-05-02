use std::fs;
use super::args::IprbArgs;
use crate::utils::sequence::read_lines;

pub fn solve(args: &IprbArgs) {
    // Args: -f, [k, m, n], -w

    // Read values from command line
    let (k, m, n) = if let Some(file) = &args.readfile {
        let lines = read_lines(file.to_str().expect("Invalid path"));

        let values: Vec<u32> = lines[0]
        .trim()
        .split_whitespace()
        .map(|v| v.parse().expect("Could not parse integer"))
        .collect();

        if values.len() != 3 {
            panic!("Expected three integers (k, m and n) in the file.");
        }
        
        //let (k, m, n) = (values[0], values[1], values[2]);
        (values[0], values[1], values[2])
    } else {
        // 
        (args.k.unwrap(),args.m.unwrap(), args.n.unwrap())
    };

     // The probability
     let result = prob(k, m, n);

     // Print on screen
     println!("{:.4}", result);


    // Write to file, if provided
    if let Some(file) = &args.writefile {
        fs::write(file, result.to_string()).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }

}

fn prob(k: u32, m: u32, n: u32) -> f64 {
    let t = k + m + n;
    let t = t as f64;

    let k = k as f64;
    let m = m as f64;
    let n = n as f64;

    let p = (k * (k - 1.0)
        + 2.0 * k * m
        + 2.0 * k * n
        + 0.75 * m * (m - 1.0)
        + m * n)
        / (t * (t - 1.0));

    p
}
