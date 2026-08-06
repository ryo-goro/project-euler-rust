use super::modular;

const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

/// Miller-Rabin implementation.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for &p in &WITNESSES {
        if n == p {
            return true;
        }

        if n.is_multiple_of(p) {
            return false;
        }
    }

    // n is odd.
    // n - 1 = d * 2^s (s > 0, d is odd)
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;

    'witness: for &a in &WITNESSES {
        // b = a^d % n
        let mut b = modular::mod_pow(a, d, n);

        if b == 1 || b == n - 1 {
            continue;
        }

        for _r in 1..s {
            // b = a^{d * 2^{_r}} % n
            b = modular::mod_mul(b, b, n);

            if b == 1 {
                return false;
            }

            if b == n - 1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}

/// Returns a sieve for the range [0, limit),
/// where `is_prime[n]` is `true` iff `n` is prime.
pub fn sieve(limit: usize) -> Vec<bool> {
    if limit <= 1 {
        return vec![false; limit];
    }

    let mut is_prime = vec![true; limit];

    is_prime[0] = false;
    is_prime[1] = false;

    for even in (4..limit).step_by(2) {
        is_prime[even] = false;
    }

    let mut odd = 3;

    while odd * odd < limit {
        if is_prime[odd] {
            for idx in (odd * odd..limit).step_by(2 * odd) {
                is_prime[idx] = false;
            }
        }

        odd += 2;
    }

    is_prime
}
