use crate::math::permutation;
use crate::math::primes;

// Reasonable searching limit.
const MAX_DIGITS: usize = 16;

// The last digit of a prime greater than 5 must be one of these.
const LAST_DIGITS: [u64; 4] = [1, 3, 7, 9];

// An 8-prime family can contain at most two non-prime numbers.
const MAX_MISS: usize = 2;

pub fn solve() -> i64 {
    let mut is_star = [false; MAX_DIGITS];
    let mut digits = [0; MAX_DIGITS];

    // Keep the smallest valid family found.
    let mut answer: Option<u64> = None;

    for num_digits in 4usize..=MAX_DIGITS {
        for last_digit in LAST_DIGITS {
            digits[num_digits - 1] = last_digit;

            // The number of replaced digits must be a multiple of 3.
            // Otherwise at least three family members are divisible by 3.
            for num_stars in (3..num_digits).step_by(3) {
                let num_fixed_digits = num_digits - num_stars - 1;
                let pow10 = 10u64.pow(num_fixed_digits as u32);

                // Marks the digit positions (excluding the last digit)
                // that are replaced together.
                is_star[..num_fixed_digits].fill(false);
                is_star[num_fixed_digits..num_digits - 1].fill(true);

                loop {
                    for fixed_digits in 0..pow10 {
                        // Skip families whose digit sum is divisible by 3.
                        if (last_digit + fixed_digits).is_multiple_of(3) {
                            continue;
                        }
    
                        // Expand `fixed_digits` into the non-replaced positions.
                        let mut remaining = fixed_digits;
    
                        for i in 0..num_digits - 1 {
                            if !is_star[i] {
                                digits[i] = remaining % 10;
                                remaining /= 10;
                            }
                        }

                        // The leading digit must not be zero.
                        if !is_star[0] && digits[0] == 0 {
                            continue;
                        }

                        // The first prime encountered is the smallest in this family.
                        let mut smallest_in_family = None;

                        // If the leading digit is replaced, 0 cannot be used.
                        let mut miss = if is_star[0] { 1 } else { 0 };
                        let start_digit = if is_star[0] { 1 } else { 0 };

                        for digit in start_digit..10 {
                            for i in 0..num_digits - 1 {
                                if is_star[i] {
                                    digits[i] = digit;
                                }
                            }

                            let candidate = to_u64(&digits[..num_digits]);

                            if !primes::is_prime(candidate) {
                                miss += 1;

                                // Too many composite numbers to reach an 8-prime family.
                                if miss > MAX_MISS {
                                    break;
                                }
                            } else if smallest_in_family.is_none() {
                                smallest_in_family = Some(candidate);
                            }
                        }
                        
                        if miss <= MAX_MISS {
                            let smallest_in_family = smallest_in_family.unwrap();

                            answer = Some(match answer {
                                Some(current) => current.min(smallest_in_family),
                                None => smallest_in_family,
                            });
                        }
                    }

                    if !permutation::next_permutation(&mut is_star[..num_digits - 1]) {
                        break;
                    }
                }
            }
        }

        if answer.is_some() {
            break;
        }
    }

    answer
        .expect("answer could not be found") as i64
}

// Converts decimal digits into an integer.
fn to_u64(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |value, &digit| 10 * value + digit)
}
