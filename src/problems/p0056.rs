use crate::math::biguint::BigUint;

const LIMIT: u64 = 100;

pub fn solve() -> i64 {
    let mut max_sum = 1;

    for a in 2..LIMIT {
        let mut pow = BigUint::one();

        for _b in 1..LIMIT {
            pow *= a;
            max_sum = max_sum.max(pow.digit_sum());
        }
    }

    max_sum as i64
}
