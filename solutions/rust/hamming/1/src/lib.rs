/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    if (s1.is_empty() && s2.is_empty()) || (s1 == s2) {
        return Some(0);
    }
    let mut ham: usize = 0;
    for (i, c) in s1.chars().enumerate() {
        for (i2, c2) in s2.chars().enumerate() {
            if i == i2 && c != c2 {
                ham += 1;
            }
        }
    }
    Some(ham)
}