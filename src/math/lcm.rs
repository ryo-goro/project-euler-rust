use crate::math::gcd;

/// Calculates the least common multiple of `a` and `b`.
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd::gcd(a, b) * b
}

pub fn lcm_many<I>(iter: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    iter.into_iter().fold(1, lcm)
}
