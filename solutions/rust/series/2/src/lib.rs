pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret_list: Vec<String> = Vec::new();
    if len == 0 {
        return vec![String::new(); digits.len() + 1];
    }
    let chars: Vec<char> = digits.chars().collect();
    for i in 0..chars.len() {
        if i + len <= chars.len() {
            let slice: String = chars[i..(i + len)].iter().collect();
            ret_list.push(slice);
        }
    }
    ret_list
}