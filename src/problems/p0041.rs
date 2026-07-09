use crate::math::permutation;
use crate::math::primes;

const CANDIDATE_LENGTHS: [usize; 2] = [7, 4];

pub fn solve() -> i64 {
    // Maximum pandigital length is 7.
    let mut digits = [0u64; 7];

    // Since the digit sum is divisible by 3 for all other pandigital lengths,
    // only 4- and 7-digit pandigital numbers can be prime.
    for n in CANDIDATE_LENGTHS {
        for i in 0..n {
            digits[i] = (n - i) as u64;
        }

        // Iterate over all `n`-digit pandigital numbers in descending order.
        loop {
            let candidate = digits[..n].iter().fold(0, |x, &d| x * 10 + d);

            if primes::is_prime(candidate) {
                return candidate as i64;
            }

            if !permutation::prev_permutation(&mut digits[..n]) {
                break;
            }
        }
    }

    unreachable!("2143 is a pandigital prime number");    
}
