use super::args::PermArgs;
use crate::utils::sequence::read_lines;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

pub fn solve(args: &PermArgs){

    let path = args
        .readfile
        .as_ref()
        .expect("Mandatory input file")
        .to_str()
        .expect("Error converting path to string");

    let lines = read_lines(path);

    let n: usize = lines
        .first()
        .expect("The file must contain at least one line")
        .trim()
        .parse()
        .expect("Could not parse n as integer");

    let nums: Vec<usize> = (1..=n).collect();
    let permutations = permute(nums);

    let total = factorial(n);

    let mut result = format!("{}\n", total);

for p in permutations {
    let line = p.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    result.push_str(&format!("{}\n", line));
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

pub fn permute(nums: Vec<usize>) -> Vec<Vec<usize>> {
    nums.iter().permutations(nums.len()).map(|p| p.into_iter().cloned().collect()).collect()
}

pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}