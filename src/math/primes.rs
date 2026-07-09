pub fn is_prime(x: u64) -> bool {
    if x <= 3 {
        return x >= 2;
    }

    if x.is_multiple_of(2) {
        return false;
    }

    let mut d = 3;

    while d <= x / d {
        if x.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }

    true
}

/// Returns a sieve for the range [0, limit), where `is_prime[n]` is `true` iff `n` is prime.
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
