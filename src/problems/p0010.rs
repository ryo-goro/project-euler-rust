use crate::math::primes;

pub fn solve() -> u64 {
    primes::sieve(2000000)
        .into_iter()
        .enumerate()
        .filter_map(|(num, is_prime)| is_prime.then_some(num as u64))
        .sum()
}
