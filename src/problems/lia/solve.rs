use super::args::LiaArgs;
use crate::utils::sequence::read_lines;
use std::fs;

pub fn solve(args: &LiaArgs) {
    // Lê os valores k (geração) e n (mínimo de Aa Bb desejados)
    let (k, n) = if let Some(file) = &args.readfile {
        let lines = read_lines(file.to_str().expect("Invalid path"));

        let values: Vec<u64> = lines[0]
            .trim()
            .split_whitespace()
            .map(|v| v.parse().expect("Could not parse integer"))
            .collect();

        if values.len() != 2 {
            panic!("Expected two integers (k and n) in the file.");
        }

        (values[0], values[1])
    } else {
        (args.k.unwrap(), args.n.unwrap())
    };

    let total_offspring = 2_u64.pow(k as u32); // Total de filhos: 2^k
    let probability = binomial_cdf_upper_tail(total_offspring, n, 0.25); // P(x ≥ n)

    println!("Resultado: {:.8}", probability);

    if let Some(file) = &args.writefile {
        fs::write(file, format!("{:.8}", probability)).unwrap_or_else(|_| {
            eprintln!("Error: Could not write to file {}", file.display());
        });
    }
}

/// Soma as probabilidades binomiais de obter pelo menos `min_successes` sucessos.
fn binomial_cdf_upper_tail(trials: u64, min_successes: u64, p: f64) -> f64 {
    (min_successes..=trials)
        .map(|i| binomial_probability(trials, i, p))
        .sum()
}

/// Calcula a probabilidade binomial de obter exatamente `c` sucessos em `n` tentativas.
fn binomial_probability(n: u64, c: u64, p: f64) -> f64 {
    if c > n {
        return 0.0;
    }

    let comb = binomial_coefficient(n, c);
    let q = 1.0 - p;
    comb * p.powi(c as i32) * q.powi((n - c) as i32)
}

/// Calcula o coeficiente binomial nCk como f64 para evitar overflow.
fn binomial_coefficient(n: u64, k: u64) -> f64 {
    let k = k.min(n - k); // Usa a simetria nCk = nC(n-k)
    let mut result = 1.0;

    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }

    result
}
