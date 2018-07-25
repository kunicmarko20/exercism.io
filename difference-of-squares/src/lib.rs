pub fn square_of_sum(n: i32) -> i32 {
    let mut sum: i32 = 0;

    for i in 1..n+1 {
        sum += i;
    }

    sum.pow(2)
}

pub fn sum_of_squares(n: i32) -> i32 {
    let mut sum: i32 = 0;

    for i in 1..n+1 {
        sum += i.pow(2);
    }

    sum
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
