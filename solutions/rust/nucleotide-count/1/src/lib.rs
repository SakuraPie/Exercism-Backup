use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        return Err(nucleotide);
    }
    let valid_result = valid_dna(dna);
    let mut count = 0;
    if valid_result.0 {
        for c in dna.chars() {
            if c == nucleotide {
                count += 1;
            }
        }
        return Ok(count)
    }
    Err(valid_result.1)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let valid_result = valid_dna(dna);
    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);
    if valid_result.0 {
        for c in dna.chars() {
            if let Some(count) = counts.get_mut(&c) {
                *count += 1;
            }
        }
        return Ok(counts);
    }
    Err(valid_result.1)
}

fn valid_dna(dna: &str) -> (bool, char) {
    for c in dna.chars() {
        if c != 'A' && c != 'C' && c != 'G' && c != 'T' {
            return (false, c);
        }
    }
    (true, '0')
}