use crate::math::digits;

const N: u64 = 100;

// 100! < 100^100 = 10^200, so 200 digits are sufficient.
const MAX_DIGITS: usize = 200;

pub fn solve() -> i64 {
    // Least significant digit first.
    let mut digits = [0; MAX_DIGITS];
    digits[0] = 1;
    let mut num_digits = 1;

    for factor in 1..=N {
        for digit in &mut digits[..num_digits] {
            *digit *= factor;
        }

        num_digits = digits::propagate_carry(&mut digits, num_digits);
    }

    digits[..num_digits].iter().sum::<u64>() as i64
}
