const N: usize = 100;
const THRESHOLD: u64 = 1_000_000;

pub fn solve() -> i64 {
    // C(n, 0) = C(n, n) = 1.
    // C(n, r) = C(n - 1, r - 1) + C(n - 1, r) (1 <= r < n).
    let mut combinations = [0; N + 1];

    let mut count = 0;

    for n in 1..=N {
        let mut prev = 1;  // C(n, r - 1)

        for r in 1..n {
            let mut curr = combinations[r - 1] + combinations[r];  // C(n, r)

            if curr > THRESHOLD {
                count += 1;
                curr = THRESHOLD + 1;
            }

            combinations[r - 1] = prev;
            prev = curr;
        }
        
        combinations[n - 1] = prev;
        combinations[n] = 1;
    }

    count
}
