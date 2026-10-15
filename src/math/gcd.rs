/// Calculates the greatest common divisor of `a` and `b`.
/// 
/// Returns `a` if `b` is zero, or `b` if `a` is zero.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn gcd_many<I>(iter: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    iter.into_iter().fold(0, gcd)
}
