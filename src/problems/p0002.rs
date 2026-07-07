use std::iter;

pub fn solve() -> i64 {
    iter::successors(
        Some((0i64, 1i64)),
         |&(a, b)| Some((b, a + b))
        )
        .map(|(_, f)| f)
        .filter(|&f| f % 2 == 0)
        .take_while(|&f| f <= 4_000_000)
        .sum()
}
