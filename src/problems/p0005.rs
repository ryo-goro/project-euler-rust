use crate::math::lcm;

pub fn solve() -> i64 {
    lcm::lcm_many(1..=20) as i64
}
