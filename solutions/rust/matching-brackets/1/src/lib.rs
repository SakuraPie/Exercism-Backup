use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let mut char_map = HashMap::new();
    char_map.insert(')', '(');
    char_map.insert(']', '[');
    char_map.insert('}', '{');

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => match stack.pop() {
                Some(x) if x == *char_map.get(&c).unwrap() => {}
                _ => return false,
            },
            _ => continue,
        }
    }
    stack.is_empty()
}