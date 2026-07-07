use crate::math::primes;

const LIMIT: i64 = 1000;

pub fn solve() -> i64 {
    let mut a_res = 0;
    let mut b_res = 0;
    let mut max_len = 0;

    for b in 0..=LIMIT {
        if !primes::is_prime(b as u64) {
            continue;
        }

        for a in -LIMIT + 1..LIMIT {
            // n = 0 yields b, which we've already verified is prime.
            let mut n = 1;

            // At n = b, monic_quadratic(a, b, n) = b * (b + a + 1).
            // This is composite unless b + a + 1 = 1, i.e. a = -b.
            // But then monic_quadratic(-b, b, 1) = 1, which is not prime.
            // Therefore, the sequence cannot continue past n = b.
            while n <= b {
                let quadratic = monic_quadratic(a, b, n);

                if quadratic < 0 || !primes::is_prime(quadratic as u64) {
                    break;
                }

                n += 1;
            }

            if n > max_len {
                max_len = n;
                a_res = a;
                b_res = b;
            }
        }
    }

    a_res * b_res
}

fn monic_quadratic(a: i64, b: i64, n: i64) -> i64 {
    n * n + a * n + b
}
