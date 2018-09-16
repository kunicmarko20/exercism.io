/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.chars().count() != s2.chars().count() {
        return None;
    }

    let string_as_vector = s2.chars().collect::<Vec<_>>();
    let mut distance = 0;

    for (index, value) in s1.chars().enumerate() {
        if &value != string_as_vector.get(index).unwrap() {
            distance += 1;
        }
    }

    Some(distance)
}
