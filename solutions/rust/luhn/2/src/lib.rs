pub fn is_valid(code: &str) -> bool {
    let mut arr: Vec<u32> = vec![];
    for c in code.chars() {
        if c.is_numeric() {
            if let Some(n) = c.to_digit(10) {
                arr.push(n);
            }
        } else if c != ' ' {
            return false;
        }
    }
    if arr.len() < 2 {
        return false;
    }
    let mut double_arr: Vec<u32> = vec![];

    for (i, n) in arr.iter().enumerate() {
        if (i % 2 == 0 && arr.len() % 2 == 0) || (i % 2 != 0 && arr.len() % 2 != 0) {
            println!("i: {}\tn: {}", i, n);
            double_arr.push(if n * 2 < 10 { n * 2 } else { n * 2 - 9 });
        } else {
            double_arr.push(*n);
        }
    }
    let sum = double_arr.iter().sum::<u32>();
    sum % 10 == 0
}