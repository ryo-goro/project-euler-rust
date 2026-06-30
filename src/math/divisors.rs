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
