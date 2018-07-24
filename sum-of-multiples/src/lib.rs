pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    for number in 1..limit {
        if let Some(num) = factor_in_number(factors, number) {
            sum += num;
        }
    }

    sum
}

fn factor_in_number(factors: &[u32], number: u32) -> Option<u32>
{
    for factor in factors {
        if number % factor == 0 {
            return Some(number);
        }
    }

    None
}