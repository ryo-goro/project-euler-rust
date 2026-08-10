pub fn solve() -> i64 {
    // An `n`-digit number satisfies `10^(n - 1) <= base^n < 10^n`.
    // The upper bound implies `base < 10`, so `base` = `1`, ..., `9`.
    //
    // From the lower bound:
    //   `n - 1 <= n * log10(base)`
    // hence:
    //   `n <= 1 / (1 - log10(base))`.
    //
    // Therefore, for each base, the number of valid exponents is the
    // floor of this upper bound.
    (1..10)
        .map(|base| (1.0 / (1.0 - (base as f64).log10())).floor() as i64)
        .sum()
}
