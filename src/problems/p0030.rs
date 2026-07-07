const UPPER_BOUND: usize = 6 * 9usize.pow(5);

pub fn solve() -> i64 {
    let fifth_powers: [usize; 10] = std::array::from_fn(|n| n.pow(5));

    let mut total = 0;

    // Let x be an n-digit integer that can be written as the sum of the fifth powers of its digits.
    // Then 10^(n - 1) <= x <= n * 9^5.
    // This inequality holds only for n <= 6, so it suffices to check x <= 6 * 9^5.
    for target in 2..=UPPER_BOUND {
        let mut remaining = target;
        let mut sum = 0;

        while remaining > 0 {
            sum += fifth_powers[remaining % 10];
            remaining /= 10;
        }

        if sum == target {
            total += target;
        }
    }

    total as i64
}
