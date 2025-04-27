use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Pega os argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: add_problem <problem_name>");
        std::process::exit(1);
    }
    let problem_name = &args[1];

    // Verifica se está na raiz de um projeto Cargo
    if !Path::new("Cargo.toml").exists() {
        eprintln!("Error: Cargo.toml not found. Please run this from the root of your project.");
        std::process::exit(1);
    }

    // Cria o diretório src/problems/<problem_name>
    let problem_dir = Path::new("src/problems").join(problem_name);
    if problem_dir.exists() {
        eprintln!("Error: Problem folder '{}' already exists.", problem_name);
        std::process::exit(1);
    }

    fs::create_dir_all(&problem_dir).expect("Could not create problem directory");

    // Cria arquivos em branco: args.rs, solve.rs, mod.rs
    let files = ["args.rs", "solve.rs", "mod.rs"];
    for file in &files {
        let filepath = problem_dir.join(file);
        let mut f = fs::File::create(&filepath).expect("Could not create file");
        // Opcional: escrever algo inicial, tipo um comentário
        writeln!(f, "// {} - {}", problem_name, file).unwrap();
    }

    println!("Problem '{}' scaffold created successfully!", problem_name);
}
