pub fn raindrops(n: u32) -> String {
    let mut ret = "".to_string();
    if n % 3 == 0 {
        ret += "Pling";
    }
    if n % 5 == 0 {
        ret += "Plang";
    }
    if n % 7 == 0 {
        ret += "Plong";
    }
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        ret = n.to_string()
    }
    ret
}
