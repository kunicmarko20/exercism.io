pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut s = digits.to_string();

    while s.chars().count() >= len {
        result.push(s[..len].to_string());
        s.remove(0);
    }

    result
}
