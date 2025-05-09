use super::args::PrtmArgs;
use std::fs;
use crate::utils::proteins::calculate_protein_mass;
use crate::utils::sequence::read_lines;

pub fn solve(args: &PrtmArgs) {
    // Lê o caminho do arquivo
    let path_str = args.readfile
        .as_ref()
        .expect("Mandatory input file")
        .to_str()
        .expect("Error converting path to string");

    // Lê a sequência da proteína (apenas a primeira linha)
    let lines = read_lines(path_str);
    if lines.is_empty() {
        eprintln!("Error: input file is empty");
        return;
    }
    let protein = lines[0].trim().to_string();

    // Calcula a massa da proteína
    match calculate_protein_mass(&protein) {
        Ok(mass) => {
            println!("Protein mass: {:.4}", mass);

            // Exporta o valor da massa para arquivo (se especificado)
            if let Some(file) = &args.writefile {
                fs::write(file, format!("{:.4}", mass)).unwrap_or_else(|_| {
                    eprintln!("Error: Could not write to file {}", file.display());
                });
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
