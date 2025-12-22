/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    if isbn.len() != 10 || !valid_type(&isbn) {
        return false;
    }
    let mut isbn_vec = vec![];
    for c in isbn.chars() {
        if c != 'X' {
            isbn_vec.push(c.to_digit(10).unwrap());
        } else {
            isbn_vec.push(10);
        }
    }
    let mut sum = 0;
    for i in 1..=10 {
        sum += isbn_vec[i - 1] * (11 - i as u32)
    }
    sum % 11 == 0
}

fn valid_type(s: &str) -> bool {
    let mut m = s.to_string();
    if s.ends_with("X") {
        m = m.replace("X", "");
    }
    let value = m.parse::<u32>();
    match value {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}