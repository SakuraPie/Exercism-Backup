use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    let candidate = candidate.replace(" ", "").replace("-", "").to_ascii_lowercase();
    for c in candidate.chars() {
        if !seen.insert(c) {
            return false
        }
    }
    true
}