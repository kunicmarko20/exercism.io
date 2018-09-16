pub fn square_of_sum(n: i32) -> i32 {
    (1..=n).sum::<i32>().pow(2)
}

pub fn sum_of_squares(n: i32) -> i32 {
    (1..=n).map(|number| number.pow(2)).sum()
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
