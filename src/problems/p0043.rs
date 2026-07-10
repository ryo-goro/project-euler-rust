use crate::math::permutation;

const PRIMES: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

pub fn solve() -> i64 {
    // Skip numbers with a leading 0.
    let mut digits = [1, 0, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut total = 0;

    loop {
        if PRIMES
            .iter()
            .enumerate()
            .all(|(i, &prime)| to_u64(&digits[i + 1..i + 4]).is_multiple_of(prime))
        {
            total += to_u64(&digits);
        }

        if !permutation::next_permutation(&mut digits) {
            break;
        }
    }

    total as i64
}

fn to_u64(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |a, &b| 10 * a + b)
}
