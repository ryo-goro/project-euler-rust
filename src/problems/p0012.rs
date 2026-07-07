use crate::math::divisors;

const THRESHOLD: usize = 500;

pub fn solve() -> i64 {
    let mut n = 1;

    loop {
        let num_divisors_n = divisors::num_divisors(n);

        // Let T(n) = n(n + 1) / 2, then:
        // T(2n - 1) = n(2n - 1)
        // T(2n) = n(2n + 1)
        for odd_factor in [2 * n - 1, 2 * n + 1] {
            // Since n and (2n ± 1) are coprime,
            // num_divisors(n(2n ± 1)) = num_divisors(n) * num_divisors(2n ± 1).
            if num_divisors_n * divisors::num_divisors(odd_factor) > THRESHOLD {
                return (n * odd_factor) as i64;
            }
        }

        n += 1;
    }
}
