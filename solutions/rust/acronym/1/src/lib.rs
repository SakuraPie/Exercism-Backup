pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let processed: String = phrase.chars()
        .map(|c| match c {
            '-' | '_' => ' ',
            _ => c,
        })
        .collect();

    for word in processed.split_whitespace() {
        if word.is_empty() {
            continue;
        }
        if let Some(first) = word.chars().find(|c| c.is_alphabetic()) {
            result.push(first.to_ascii_uppercase());
        }
        let mut chars = word.chars();
        if let Some(mut prev) = chars.next() {
            for c in chars {
                if c.is_uppercase() && prev.is_lowercase() {
                    result.push(c);
                }
                prev = c;
            }
        }
    }
    result
}