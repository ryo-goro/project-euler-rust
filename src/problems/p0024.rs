use crate::math::permutation;

const TARGET: usize = 1000000;
const PERM_LEN: usize = 10;

pub fn solve() -> u64 {
    let digits = permutation::nth_permutation(PERM_LEN, TARGET - 1);

    digits.iter().fold(0, |x, &d| x * 10 + d as u64)
}
