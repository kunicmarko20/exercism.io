const MAX_NUMBER: u32 = 1000;

pub fn find() -> Option<u32> {
    for a in 1..MAX_NUMBER {
        for b in a+1..MAX_NUMBER {
            if is_result_signed(a, b) {
                continue;
            }

            let c = MAX_NUMBER - a - b;

            if is_valid(a, b, c) {
                return Some(a * b * c);
            }
        }
    };

    None
}

fn is_result_signed(a: u32, b: u32) -> bool {
    a + b > MAX_NUMBER
}

fn is_valid(a: u32, b: u32, c: u32) -> bool {
    a * a + b * b == c * c
}
