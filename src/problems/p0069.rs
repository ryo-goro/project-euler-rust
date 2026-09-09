use crate::math::primes;

const N: u64 = 1_000_000;

pub fn solve() -> i64 {
    // Let p_{1}, p_{2}, ..., p_{m} be all distinct prime factors of n > 1.
    // Then n / φ(n) = (1 + 1 / (p_{1} - 1)) * (1 + 1 / (p_{2} - 1)) * ... * (1 + 1 / (p_{m} - 1)).
    // Therefore, to maximize the ratio;
    // - make m as big as possible.
    //   In particular, n should not have square factors.
    // - make each prime factor of n as small as possible.

    // See docs/math/number_theory/09_euler_totient_function.md
    // for more information of φ(n).

    let mut res = 2;

    for p in (3..).step_by(2) {
        if !primes::is_prime(p) {
            continue;
        }

        if res * p > N {
            break;
        }

        res *= p;
    }

    res as i64
}
