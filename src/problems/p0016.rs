use crate::math::digits;

const POWER: usize = 1000;

// Multiply by 2^60 at a time so intermediate digit values still fit in u64.
const CHUNK_BITS: usize = 60;

// 2^1000 has 302 decimal digits, so POWER digits are more than enough.
const MAX_DIGITS: usize = POWER;

pub fn solve() -> i64 {
    // Least significant digit first.
    let mut digits = [0; MAX_DIGITS];
    digits[0] = 1;

    let mut num_digits = 1;

    let num_chunks = POWER / CHUNK_BITS;
    let remaining_bits = POWER % CHUNK_BITS;

    for _ in 0..num_chunks {
        multiply_digits(&mut digits[..num_digits], 1 << CHUNK_BITS);
        num_digits = digits::propagate_carry(&mut digits, num_digits);
    }

    multiply_digits(&mut digits[..num_digits], 1 << remaining_bits);
    num_digits = digits::propagate_carry(&mut digits, num_digits);

    digits[..num_digits].iter().sum::<u64>() as i64
}

fn multiply_digits(digits: &mut [u64], multiplier: u64) {
    for digit in digits {
        *digit *= multiplier;
    }
}
