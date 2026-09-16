use crate::math::gcd;

const MAX_DENOMINATOR: u64 = 12_000;

pub fn solve() -> i64 {
    let mut count = 0;

    for d in 2..=MAX_DENOMINATOR {
        // 1 / 3 < n / d < 1 / 2 <=> d / 3 < n < d / 2.
        for n in d / 3 + 1..(d + 1) / 2 {
            if gcd::gcd(d, n) == 1 {
                count += 1;
            }
        }
    }

    count
}
