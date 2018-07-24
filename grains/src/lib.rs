pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;

    for number in 0..64 {
        sum += 2_u64.pow(number)
    }
    
    sum
}
