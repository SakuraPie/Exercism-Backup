#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }
    let mut all_zero = true;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        if digit != 0 {
            all_zero = false;
        }
    }
    if all_zero {
        return Ok(vec![0]);
    }
    let mut value: u32 = 0;
    for &digit in number {
        value = value
            .checked_mul(from_base)
            .ok_or(Error::InvalidDigit(digit))?
            .checked_add(digit)
            .ok_or(Error::InvalidDigit(digit))?;
    }

    let mut digits = Vec::new();
    let mut n = value;

    while n > 0 {
        digits.push(n % to_base);
        n /= to_base;
    }
    digits.reverse();
    Ok(digits)
}