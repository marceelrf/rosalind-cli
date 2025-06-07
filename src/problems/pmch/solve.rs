use crate::utils::sequence::{read_fasta,count_rna_bases};
use super::args::PmchArgs;
use std::fs::File;
use std::io::Write;



pub fn solve(args: &PmchArgs) {

    // Read the path to fasta file
    let path = args
            .readfile
            .as_ref().expect("Mandatory input file")
            .to_str()
            .expect("Error converting path to string");

    let seqs = read_fasta(path);

    if seqs.len() != 1 {
        panic!("Expected exactly one sequence in the FASTA file.");
    }

    let seq = seqs.values().next().unwrap();

    // Count the number of A,U and C,G
    let counts = count_rna_bases(&seq);

    let a = *counts.get(&'A').unwrap_or(&0);
    let u = *counts.get(&'U').unwrap_or(&0);
    let c = *counts.get(&'C').unwrap_or(&0);
    let g = *counts.get(&'G').unwrap_or(&0);
    
    if a != u || c != g {
        panic!("❌ RNA cannot be perfectly paired: A={}, U={}, C={}, G={}", a, u, c, g);
    }

    let result = factorial(a as u128) * factorial(c as u128);

    if args.printresult {
        print!("{}", result);
    }

    // Save to file if specified
    if let Some(write_path) = &args.writefile {
        let mut file = File::create(write_path)
            .expect("Failed to create output file");
        file.write_all(result.to_string().as_bytes())
            .expect("Failed to write to file");
    }

}
pub fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

// pub fn validate_rna_pairing(seq: &str) -> Result<HashMap<char, usize>, String> {
//     let mut counts = HashMap::new();

//     for base in seq.chars() {
//         match base {
//             'A' | 'U' | 'C' | 'G' => {
//                 *counts.entry(base).or_insert(0) += 1;
//             }
//             _ => return Err(format!("Invalid RNA base: {}", base)),
//         }
//     }

//     let a = *counts.get(&'A').unwrap_or(&0);
//     let u = *counts.get(&'U').unwrap_or(&0);
//     let c = *counts.get(&'C').unwrap_or(&0);
//     let g = *counts.get(&'G').unwrap_or(&0);

//     if a != u || c != g {
//         return Err(format!(
//             "Unbalanced base counts: A={} U={} C={} G={}",
//             a, u, c, g
//         ));
//     }

//     Ok(counts)
// }