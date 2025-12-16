pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret_list: Vec<String> = Vec::new();
    for (index, _c) in digits.chars().enumerate() {
        if index + len <= digits.len() {
            ret_list.push(digits[index..(index + len)].to_string())
        }
    }
    ret_list
}