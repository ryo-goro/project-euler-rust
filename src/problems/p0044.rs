pub fn solve() -> i64 {
    let mut min_diff = u64::MAX;

    let mut i = 2;

    // The minimum possible difference for this i is p_i - p_{i - 1} = 3*i - 2.
    // Stop when even this difference cannot improve the current minimum.
    while ((3 * i - 2) as u64) < min_diff {
        let p_i = pentagonal_number(i);

        for j in (1..i).rev() {
            let p_j = pentagonal_number(j);
            let diff = p_i - p_j;

            if diff >= min_diff {
                // Decreasing j only increases diff, so no better candidate exists.
                break;
            }

            let sum = p_i + p_j;

            if is_pentagonal_number(diff) && is_pentagonal_number(sum) {
                min_diff = diff;

                // Further j values cannot produce a smaller difference.
                break;
            }
        }

        i += 1;
    }

    min_diff as i64
}

fn pentagonal_number(n: usize) -> u64 {
    let n = n as u64;

    n * (3 * n - 1) / 2
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
