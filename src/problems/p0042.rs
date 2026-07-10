// Comma-separated list of quoted words.
const WORDS: &str = include_str!("../../data/p0042_words.txt");

pub fn solve() -> i64 {
    WORDS
        .split(',')
        .map(|quoted_word| {
            quoted_word
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
        })
        .map(word_value)
        .filter(|&value| is_triangle_number(value))
        .count() as i64
}

fn word_value(word: &str) -> u64 {
    word
        .as_bytes()
        .iter()
        .map(|&byte| (byte - b'A' + 1) as u64)
        .sum()
}

fn is_triangle_number(x: u64) -> bool {
    if x == 0 {
        return false;
    }

    // If x > 0 is a triangle number, there exists an integer n > 0 such that x = n * (n + 1) / 2.
    // Solving this quadratic equation for n gives n = (-1 + sqrt(1 + 8 * x)) / 2.
    // Since 1 + 8 * x is always odd, x is a triangle number iff 1 + 8 * x is a perfect square.

    let radicand = 1 + 8 * x;
    let root = radicand.isqrt();

    root * root == radicand
}
