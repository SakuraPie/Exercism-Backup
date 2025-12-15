use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ret = HashSet::new();
    let word_lower = word.to_lowercase();

    let mut word_collect: Vec<char> = word_lower.chars().collect();
    word_collect.sort_unstable();

    for &letter in possible_anagrams {
        if word_lower.len() != letter.len() {
            continue;
        }
        if letter.to_lowercase() == word_lower {
            continue;
        }
        let mut letter_collect: Vec<char> = letter.to_lowercase().chars().collect();
        letter_collect.sort_unstable();

        if letter_collect == word_collect {
            ret.insert(letter);
        }
    }
    ret
}
