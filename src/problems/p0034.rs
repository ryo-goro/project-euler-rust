const FACTORIALS: [u64; 10] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
    40320,
    362880,
];

const UPPER_BOUND: u64 = 7 * FACTORIALS[9];

pub fn solve() -> i64 {
    let mut total = 0;

    // Let x be an n-digit integer (n >= 2) that can be written as the sum of the factorials of its digits.
    // Then 10^(n-1) <= x <= n * 9!.
    // This inequality holds only for n <= 7, so it suffices to check 10 <= x <= 7 * 9!.
    for x in 10..=UPPER_BOUND {
        let mut sum = 0;
        let mut remaining = x;

        while remaining > 0 {
            let digit = remaining % 10;
            sum += FACTORIALS[digit as usize];
            remaining /= 10;
        }

        if sum == x {
            total += x;
        }
    }

    total as i64
}
