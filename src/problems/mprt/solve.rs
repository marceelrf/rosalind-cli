use super::args::MprtArgs;
use crate::utils::sequence::read_lines;
use crate::utils::proteins::find_nglycosylation_sites;
use reqwest::blocking::get;
use std::fs;

pub fn solve(args: &MprtArgs) {
    let path_str = args
        .readfile
        .as_ref()
        .expect("Mandatory input file")
        .to_str()
        .expect("Error converting path to string");

    let lines = read_lines(path_str);

    let mut output = String::new(); // <- acumula o resultado

    for raw_id in lines.iter() {
        let id = raw_id.trim().split('_').next().unwrap().to_string();
        let fasta_url = format!("http://www.uniprot.org/uniprot/{}.fasta", id);

        match get(&fasta_url) {
            Ok(response) => {
                if response.status().is_success() {
                    let content = response.text().unwrap_or_else(|_| ">Error reading content\n".to_string());

                    let lines: Vec<&str> = content.lines().collect();
                    let sequence: String = lines.iter()
                        .skip(1)
                        .map(|l| l.trim())
                        .collect();

                    let positions = find_nglycosylation_sites(&sequence);
                    if !positions.is_empty() {
                        let formatted: Vec<String> = positions.iter().map(|p| p.to_string()).collect();
                        let result = format!("{}\n{}\n", raw_id, formatted.join(" "));
                        println!("{}", result); // também imprime na tela
                        output.push_str(&result); // adiciona ao output
                    }
                } else {
                    eprintln!("Failed to fetch {} (status: {})", raw_id, response.status());
                }
            }
            Err(e) => {
                eprintln!("Error downloading {}: {}", raw_id, e);
            }
        }
    }

    // Se o argumento -w foi passado, salva o output no arquivo
    if let Some(file) = &args.writefile {
        if let Err(e) = fs::write(file, output) {
            eprintln!("Failed to write output to {}: {}", file.display(), e);
        }
    }
}
