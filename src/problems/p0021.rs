use crate::math::divisors;

const LIMIT: u64 = 10000;

pub fn solve() -> i64 {
    let mut total = 0;

    for n in 1..LIMIT {
        let partner = divisors::sum_of_proper_divisors(n);

        if partner > n && n == divisors::sum_of_proper_divisors(partner) {
            total += n + partner;
        }
    }

    total as i64
}
