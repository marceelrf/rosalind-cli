fn main() {
    let dna = "ATGCGTAGCTAG";
    let bases: Vec<char> = dna.chars().collect();

    for i in (0..bases.len()).step_by(3) {
        let codon: String = bases[i..std::cmp::min(i + 3, bases.len())].iter().collect();
        println!("Codon: {}", codon);
    }
}