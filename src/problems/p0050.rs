use crate::math::primes;

const LIMIT: u64 = 1_000_000;

pub fn solve() -> i64 {
    let is_prime = primes::sieve(LIMIT as usize);

    // primes[0] = 0, primes[i] = the i-th prime number (i > 0).
    let mut primes = vec![0];

    primes.extend(
        is_prime
            .iter()
            .enumerate()
            .filter_map(|(p, &b)| b.then_some(p as u64))
    );

    let last_prime_idx = primes.len() - 1;

    let mut first = 1;
    let mut last = 1;

    // 2 is prime, and can be written as the sum of 1 consecutive prime number.
    let mut res = 2;
    let mut base_sum = 0;

    // Maintain the shortest window of the current best length.
    while last <= last_prime_idx {
        // Sum of the current window [first, last],
        // i.e., primes[first] + primes[first + 1] + ... + primes[last].
        base_sum += primes[last] - primes[first - 1];

        if base_sum >= LIMIT {
            break;
        }

        let mut trial_sum = base_sum;

        for i in last + 1..=last_prime_idx {
            trial_sum += primes[i];

            if trial_sum >= LIMIT {
                break;
            }

            if is_prime[trial_sum as usize] {
                res = trial_sum;
                base_sum = trial_sum;

                // If a longer valid sequence is found, extend the minimum window length.
                // Shorter windows cannot improve the current best result.
                last = i;
            }
        }

        // Advance both ends of the minimum window.
        first += 1;
        last += 1;
    }

    res as i64
}
