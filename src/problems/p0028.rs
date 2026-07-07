const SIZE: usize = 1001;

pub fn solve() -> i64 {
    // 21 22 23 24 25
    // 20  7  8  9 10
    // 19  6  1  2 11
    // 18  5  4  3 12
    // 17 16 15 14 13
    //
    // - To bottom right: 3, 13, 31, ..., 4k^2 - 2k + 1 (k = 1, 2, ...)
    // - To bottom left: 5, 17, 37, ..., 4k^2 + 1 (k = 1, 2, ...)
    // - To upper left: 7, 21, 43, ..., 4k^2 + 2k + 1 (k = 1, 2, ...)
    // - To upper right: 9, 25, 47, ..., 4k^2 + 4k + 1 (k = 1, 2, ...)
    //
    // The sum of the above 4 terms is 16k^2 + 4k + 4.
    // Therefore the diagonal sum is:
    // 1 + Σ(16k² + 4k + 4) = (16n^3 + 30n^2 + 26n + 3) / 3 (n = 500)

    let layers = (SIZE / 2) as i64;
    (16 * layers * layers * layers + 30 * layers * layers + 26 * layers + 3) / 3
}
