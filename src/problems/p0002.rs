use std::iter;

pub fn solve() -> u64 {
    iter::successors(
        Some((0u64, 1u64)),
         |&(a, b)| Some((b, a + b))
        )
        .map(|(_, f)| f)
        .filter(|&f| f % 2 == 0)
        .take_while(|&f| f <= 4_000_000)
        .sum()
}
