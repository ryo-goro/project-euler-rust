use crate::math::primes;

pub fn solve() -> u64 {
    (2..).filter(|&x| primes::is_prime(x)).nth(10000).unwrap()
}
