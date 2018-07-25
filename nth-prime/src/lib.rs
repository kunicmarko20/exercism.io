pub fn nth(n: u32) -> Option<u32> {
    if n < 1 {
        return None;
    }

    if n < 3 {
        return Some(n+1);
    }

    let mut primes: Vec<u32> = vec![2, 3];
    let mut number = 4;

    while primes.len() != n as usize {
        if is_prime(number, &primes) {
            primes.push(number);
        } 

        number += 1;
    }

    Some(*primes.last().unwrap())
}

fn is_prime(number: u32, primes: &Vec<u32>) -> bool {
    for prime in primes {
        if number % prime == 0 {
            return false;
        }
    }

    true
}