pub fn num_divisors(mut n: u64) -> usize {
    assert!(n > 0);

    let mut num_divisors = 1;

    while n.is_multiple_of(2) {
        n /= 2;
        num_divisors += 1;
    }

    let mut d = 3;

    while d <= n / d {
        let mut exponent = 0;

        while n.is_multiple_of(d) {
            n /= d;
            exponent += 1;
        }

        num_divisors *= exponent + 1;
        d += 2;
    }

    if n > 1 {
        // Remaining prime factor.
        num_divisors *= 2;
    }

    num_divisors
}

pub fn sum_of_divisors(mut n: u64) -> u64 {
    assert!(n > 0);

    // Contribution of the prime factor 2.
    let mut sum = 2;

    while n.is_multiple_of(2) {
        n /= 2;
        sum *= 2;
    }

    sum -= 1;

    let mut p = 3;

    while p <= n / p {
        let mut power = p;

        while n.is_multiple_of(p) {
            n /= p;
            power *= p;
        }

        // Multiply by (p^(e+1) - 1) / (p - 1).
        sum *= (power - 1) / (p - 1);

        p += 2;
    }

    if n > 1 {
        // Remaining prime factor.
        sum *= n + 1;
    }

    sum
}

pub fn sum_of_proper_divisors(n: u64) -> u64 {
    sum_of_divisors(n) - n
}
