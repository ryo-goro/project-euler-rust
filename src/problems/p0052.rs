const N: u64 = 6;

pub fn solve() -> i64 {
    for num_digits in 1.. {
        let min = 10u64.pow(num_digits - 1);
        let max = min * 10 / N;

        // `x >= 10^{num_digits - 1}` and `N * x < 10^num_digits` are required
        // so that x, 2x, ..., Nx all have `num_digits` digits.
        for x in min..=max {
            let target = count_digits(x);

            if (2..=N).all(|n| count_digits(n * x) == target) {
                return x as i64;
            }
        }
    }

    unreachable!();
}

fn count_digits(mut x: u64) -> [usize; 10] {
    let mut counts = [0; 10];

    while x > 0 {
        counts[(x % 10) as usize] += 1;
        x /= 10;
    }

    counts
}
