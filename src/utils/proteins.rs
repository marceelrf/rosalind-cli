use std::collections::HashMap;

pub fn aa_mass() -> HashMap<char, f64> {
    HashMap::from([
        ('A', 71.03711),
        ('C', 103.00919),
        ('D', 115.02694),
        ('E', 129.04259),
        ('F', 147.06841),
        ('G', 57.02146),
        ('H', 137.05891),
        ('I', 113.08406),
        ('K', 128.09496),
        ('L', 113.08406),
        ('M', 131.04049),
        ('N', 114.04293),
        ('P', 97.05276),
        ('Q', 128.05858),
        ('R', 156.10111),
        ('S', 87.03203),
        ('T', 101.04768),
        ('V', 99.06841),
        ('W', 186.07931),
        ('Y', 163.06333),
    ])
}
/// Calculate the protein mass give the protein sequence
pub fn calculate_protein_mass(protein: &str) -> Result<f64, String> {
    let mass = aa_mass();
    let mut protein_mass = 0.0;
    
    for aa in protein.chars() {
        protein_mass += *mass.get(&aa)
            .ok_or_else(|| format!("Unknown amino acid: {}", aa))?;
    }
    
    Ok(protein_mass)
}
/// Returns the (1-based) positions where the N-glycosylation motif occurs in the sequence
pub fn find_nglycosylation_sites(seq: &str) -> Vec<usize> {
    let seq = seq.as_bytes();
    let mut positions = Vec::new();

    if seq.len() < 4 {
        return positions;
    }

    for i in 0..(seq.len() - 3) {
        let a = seq[i] as char;
        let b = seq[i + 1] as char;
        let c = seq[i + 2] as char;
        let d = seq[i + 3] as char;

        if a == 'N' && b != 'P' && (c == 'S' || c == 'T') && d != 'P' {
            positions.push(i + 1); // 1-based
        }
    }

    positions
}
