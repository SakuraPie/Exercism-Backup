pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut current = n;
    let mut count = 0;
    while current != 1 {
        if current % 2 == 0 {
            current /= 2
        } else {
            current = current * 3 + 1
        }
        count += 1
    }
    Some(count)
}