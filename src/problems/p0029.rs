const MAX: usize = 100;

pub fn solve() -> i64 {
    let mut checked = [false; MAX + 1];
    let mut total = 0;

    for a in 2..=MAX {
        if checked[a] {
            continue;
        }

        // Since values are processed in ascending order, `a` is not a perfect power.
        // Therefore, if a^p = b^q for some integers `b`, `p`, and `q`, then `b` must also be a power of `a`.
        checked[a] = true;
        let mut pow_a = a * a;

        // Since a is not a perfect power, if a^2 > MAX, there is no b that fulfills both a < b <= MAX and a^p = b^q.
        // So we can count a^2, a^3, ..., a^MAX without worrying about the duplication.
        if pow_a > MAX {
            total += MAX - 1;
            continue;
        }

        // If a^2 <= MAX, a, a^2, a^3, ..., a^k (where k fulfills a^k <= MAX < a^{k+1})
        // should be counted carefully without duplication.

        let mut max_exponent = 1;

        while pow_a <= MAX {
            checked[pow_a] = true;
            max_exponent += 1;
            pow_a *= a;
        }

        let len = max_exponent * MAX + 1;
        let mut used_exponents = vec![false; len];

        for b in 2..=MAX {
            for c in 1..=max_exponent {
                used_exponents[b * c] = true;
            }
        }

        total += used_exponents.iter().filter(|&&used| used).count();
    }

    total as i64
}
