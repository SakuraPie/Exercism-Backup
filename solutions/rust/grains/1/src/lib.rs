pub fn square(s: u32) -> u128 {
    if s > 64 { panic!() };
    2_u128.pow(s - 1)
}

pub fn total() -> u128 {
    2_u128.pow(64) - 1
}
