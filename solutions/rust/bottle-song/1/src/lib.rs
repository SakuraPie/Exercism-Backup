use std::collections::HashMap;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if start_bottles > 10 {
        panic!("start bottles cannot > 10!")
    }
    let hm = create_map();
    let mut re = "".to_string();
    let arr = if take_down > start_bottles {
        (take_down - start_bottles + 1)..=start_bottles
    } else {
        (start_bottles - take_down + 1)..=start_bottles
    };
    for n in arr.rev() {
        re += &format!("{}\n\n", &*single_lyric(n, n - 1, &hm))
    }
    re
}

pub fn create_map() -> HashMap<u32, &'static str> {
    let mut hm = HashMap::with_capacity(10);
    hm.insert(0, "No");
    hm.insert(1, "One");
    hm.insert(2, "Two");
    hm.insert(3, "Three");
    hm.insert(4, "Four");
    hm.insert(5, "Five");
    hm.insert(6, "Six");
    hm.insert(7, "Seven");
    hm.insert(8, "Eight");
    hm.insert(9, "Nine");
    hm.insert(10, "Ten");
    hm
}

fn single_lyric(start_num: u32, end_num: u32, num_map: &HashMap<u32, &str>) -> String {
    let s1 = *num_map
        .get(&start_num)
        .expect("\x1b[31mErr: Num out of range.");
    let n1 = if start_num == 1 { "" } else { "s" };
    let s2 = *num_map
        .get(&(start_num - end_num))
        .expect("\x1b[31mErr: Num out of range.");
    let n2 = if (start_num - end_num) == 1 { "" } else { "s" };
    let s3 = *num_map
        .get(&end_num)
        .expect("\x1b[31mErr: Num out of range.");
    let n3 = if end_num == 1 { "" } else { "s" };
    format!(
        "{} green bottle{} hanging on the wall,\n\
    {} green bottle{} hanging on the wall,\n\
    And if {} green bottle{} should accidentally fall,\n\
    There'll be {} green bottle{} hanging on the wall.",
        s1,
        n1,
        s1,
        n1,
        s2.to_lowercase(),
        n2,
        s3.to_lowercase(),
        n3
    )
}
