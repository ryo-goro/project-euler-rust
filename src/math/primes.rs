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

pub fn sieve(limit: usize) -> Vec<bool> {
    if limit <= 1 {
        return vec![false; limit];
    }

    let mut is_prime = vec![true; limit];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut even = 4;

    while even < limit {
        is_prime[even] = false;
        even += 2;
    }

    let mut odd = 3;

    while odd * odd < limit {
        if is_prime[odd] {
            let mut idx = odd * odd;
            
            while idx < limit {
                is_prime[idx] = false;

                idx += odd;
            }
        }

        odd += 2;
    }

    is_prime
}
