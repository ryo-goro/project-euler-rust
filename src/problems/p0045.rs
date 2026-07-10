pub fn solve() -> i64 {
    // Every hexagonal number is also triangular:
    // H(n) = T(2n - 1).
    // Therefore, we only need to search hexagonal numbers.
    
    // H(143) = P(165) = T(285) = 40755 is the known solution,
    // so start searching from the next hexagonal number.
    (144..)
        .map(hexagonal_number)
        .find(|&hex| is_pentagonal_number(hex))
        .unwrap() as i64
}

fn hexagonal_number(n: usize) -> u64 {
    let n = n as u64;

    n * (2 * n - 1)
}

fn is_pentagonal_number(x: u64) -> bool {
    // If x > 0 is a pentagonal number, there exists an integer n > 0 such that x = n * (3 * n - 1) / 2.
    // Solving this quadratic equation for n gives n = (1 + sqrt(1 + 24 * x)) / 6.
    // Therefore, x is a pentagonal number iff 1 + 24 * x is a perfect square
    // and 1 + sqrt(1 + 24 * x) is divisible by 6.

    let radicand = 1 + 24 * x;
    let root = radicand.isqrt();

    root * root == radicand && (1 + root).is_multiple_of(6)
}
