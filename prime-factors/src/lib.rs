pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec!();
    }

    let mut prime_factors: Vec<u64> = Vec::new();
    let mut result = n.clone();

    while result != 1 {
        let next_prime_factor = next_prime_factor(result, n).unwrap();
        result = result / next_prime_factor;
        prime_factors.push(next_prime_factor);
    }

    prime_factors
}

fn next_prime_factor(result: u64, n: u64) -> Option<u64> {
    for number in 2..n+1 {
        if result % number == 0 {
            return Some(number);
        }
    }

    None
}