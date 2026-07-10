use crate::math::primes;

pub fn solve() -> i64 {
    // 9 is the first composite odd number.
    for n in (9..).step_by(2) {
        if primes::is_prime(n) {
            continue;
        }

        if !satisfies_goldbach_other_conjecture(n) {
            return n as i64;
        }
    }

    unreachable!("Goldbach's other conjecture holds for all tested values");
}

// Returns `true` iff `odd_composite` can be written as the sum of a prime and twice a square.
// `odd_composite` should be an odd composite number.
fn satisfies_goldbach_other_conjecture(odd_composite: u64) -> bool {
    for i in 1u64.. {
        let square2 = 2 * i * i;

        if square2 >= odd_composite {
            break;
        }

        if primes::is_prime(odd_composite - square2) {
            return true;
        }
    }

    false
}
