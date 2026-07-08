use crate::math::primes;

const HEAD: [u64; 4] = [2, 3, 5, 7];
const MID: [u64; 4] = [1, 3, 7, 9];
const TAIL: [u64; 2] = [3, 7];
const RADIX: usize = MID.len();
const NUM_TRUNCATABLE_PRIMES: usize = 11;


pub fn solve() -> i64 {
    let mut count = 0;
    let mut total = 0;

    'outer: for num_digits in 2.. {
        let pow10 = 10u64.pow(num_digits - 1);

        for &head in &HEAD {
            'candidate: for mut seed in 0..RADIX.pow(num_digits - 2){
                // Convert `seed` into the middle digits of the candidate.
                // Example: num_digits = 5, head = 2, tail = 3, seed = 0 = 000 (in base 4) -> candidate = 21113
                // Example: num_digits = 6, head = 2, tail = 3, seed = 27 = 0123 (in base 4) -> candidate = 297313
                let mut candidate = head;

                for _ in 0..num_digits - 2 {
                    candidate = candidate * 10 + MID[seed % RADIX];

                    // Every prefix of a right-truncatable prime must also be prime.
                    if !primes::is_prime(candidate) {
                        continue 'candidate;
                    }

                    seed /= RADIX;
                }

                for &tail in &TAIL {
                    let candidate = candidate * 10 + tail;

                    if is_left_truncatable(candidate, pow10) {
                        count += 1;
                        total += candidate;

                        if count == NUM_TRUNCATABLE_PRIMES {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    total as i64
}

// `pow10` should be 10^{n-1} where n is the number of digits of `x`.
fn is_left_truncatable(mut x: u64, mut pow10: u64) -> bool {
    while x > 0 {
        if !primes::is_prime(x) {
            return false;
        }

        x %= pow10;
        pow10 /= 10;
    }

    true
}
