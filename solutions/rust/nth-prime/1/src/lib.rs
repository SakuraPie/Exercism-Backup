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

fn cal_prime_120000() -> Vec<u64> {
    let mut ret: Vec<u64> = vec![];
    for i in 1..=120000 {
        if factors(i).iter().len() == 1 {
            ret.push(i)
        }
    }
    ret
}

pub fn nth(n: u64) -> u64 {
    cal_prime_120000()[n as usize]
}
