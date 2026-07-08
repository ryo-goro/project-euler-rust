use crate::math::primes;

const MAX_DIGITS: usize = 6;

// Every digit of a circular prime with at least two digits must be 1, 3, 7, or 9,
// since each digit becomes the least significant digit after some rotation.
const DIGITS: [u64; 4] = [1, 3, 7, 9];

pub fn solve() -> i64 {
    // 2, 3, 5, and 7 are 1-digit primes, and they all are circular primes.
    let mut count = 4;

    let mut rotations = [0; MAX_DIGITS];
    
    for num_digits in 2..=MAX_DIGITS {
        let pow10 = 10u64.pow((num_digits - 1) as u32);

        // Generate all candidate `num_digits`-digit numbers.
        'candidate: for mut seed in 0..DIGITS.len().pow(num_digits as u32) {
            let mut candidate = 0;

            // Convert `seed` to `candidate`.
            // Example: num_digits = 3, seed = 0 = 000 (in base 4) -> candidate = 111
            // Example: num_digits = 4, seed = 27 = 0123 (in base 4) -> candidate = 9731
            for _ in 0..num_digits {
                candidate = candidate * 10 + DIGITS[seed % DIGITS.len()];
                seed /= DIGITS.len();
            }

            rotations[0] = candidate;
            let mut num_rotations = 1;

            let mut rotation = rotate(candidate, pow10);
            
            // Collect all distinct rotations of `candidate`.
            while rotation != candidate {
                // Skip this candidate if one of its rotations is smaller
                // to avoid counting the same circular prime multiple times.
                if rotation < candidate {
                    continue 'candidate;
                }

                rotations[num_rotations] = rotation;
                num_rotations += 1;
                rotation = rotate(rotation, pow10);
            }

            if rotations[..num_rotations].iter().all(|&rotation| primes::is_prime(rotation)) {
                count += num_rotations;
            }
        }
    }

    count as i64
}

// `pow10` should be 10^n where n is the number of digits of `x` including leading 0s.
// Example: rotate(123, 100) = 231
// Example: rotate(3051, 1000) = (0)513
// Example: rotate(513, 1000) = rotate((0)513, 1000) = 5130
fn rotate(x: u64, pow10: u64) -> u64 {
    (x % pow10) * 10 + x / pow10
}
