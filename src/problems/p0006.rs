pub fn solve() -> u64 {
    let x: u64 = (1..=100).sum();
    let y: u64 = (1..=100).map(|n| n * n).sum();

    x * x - y
}
