const CONSECUTIVE_COUNT: usize = 4;
const NUM_DISTINCT_PRIME_FACTORS: usize = 4;

pub fn solve() -> i64 {
    let mut consecutive = 0;

    for n in 1.. {
        if num_distinct_prime_factors(n) == NUM_DISTINCT_PRIME_FACTORS {
            consecutive += 1;

            if consecutive == CONSECUTIVE_COUNT {
                return (n - CONSECUTIVE_COUNT as u64 + 1) as i64;
            }
        } else {
            consecutive = 0;
        }
    }

    unreachable!("answer could not be found");
}

/// Returns the number of distinct prime factors of x.
fn num_distinct_prime_factors(mut x: u64) -> usize {

    if x < 2 {
        return 0;
    }

    let mut count = 0;

    if x.is_multiple_of(2) {
        count += 1;

        while x.is_multiple_of(2) {
            x /= 2;
        }
    }

    let mut d = 3;

    while d <= x / d {
        if x.is_multiple_of(d) {
            count += 1;

            while x.is_multiple_of(d) {
                x /= d;
            }
        }

        d += 2;
    }

    if x > 1 {
        count += 1;
    }

    count
}
