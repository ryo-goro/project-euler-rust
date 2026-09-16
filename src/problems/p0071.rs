use crate::math::gcd;

const MAX_DENOMINATOR: u64 = 1_000_000;
const TARGET_NUMERATOR: u64 = 3;
const TARGET_DENOMINATOR: u64 = 7;

pub fn solve() -> i64 {
    let mut numerator = 1;
    let mut denominator = TARGET_DENOMINATOR;

    // If d <= 1000000 and n / d < 3 / 7,
    // n < 3 * d / 7 <= 3 * 1000000 / 7.
    for n in 1..TARGET_NUMERATOR * MAX_DENOMINATOR / TARGET_DENOMINATOR {
        // For a fixed n, n / d increases as d decreases.
        // Therefore, the smallest d satisfying n / d < 3 / 7, which is d = n * 7 / 3 + 1,
        // gives the closest fraction to 3 / 7.
        let d = n * TARGET_DENOMINATOR / TARGET_NUMERATOR + 1;

        // n / d > numerator / denominator
        if n * denominator > numerator * d {
            numerator = n;
            denominator = d;
        }
    }

    (numerator / gcd::gcd(numerator, denominator)) as i64
}
