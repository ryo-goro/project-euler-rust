use crate::math::gcd;

/// Calculates the least common multiple of `a` and `b`.
/// 
/// Returns zero if either `a` or `b` is zero.
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }

    a / gcd::gcd(a, b) * b
}

pub fn lcm_many<I>(iter: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    iter.into_iter().fold(1, lcm)
}
