pub fn solve() -> i64 {
    let x: i64 = (1..=100).sum();
    let y: i64 = (1..=100).map(|n| n * n).sum();

    x * x - y
}
