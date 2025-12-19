pub fn egg_count(display_value: u32) -> usize {
    let mut b_arr: Vec<usize> = vec![];
    let mut current = display_value;
    let mut count = 0;
    while current / 2 != 0 {
        b_arr.push((current % 2) as usize);
        current /= 2;
    }
    if display_value >= 1 {
        b_arr.push(1);
    } else {
        b_arr.push(0);
    }
    for i in b_arr {
        if i == 1 {
            count += 1;
        }
    }
    count
}