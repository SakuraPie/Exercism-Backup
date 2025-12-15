pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    while n % 2 == 0 {
        result.push(2);
        n /= 2;
    }
    let mut divisor = 3;
    while divisor * divisor <= n {
        while n % divisor == 0 {
            result.push(divisor);
            n /= divisor;
        }
        divisor += 2;
    }
    if n > 2 { result.push(n) };
    result
}
