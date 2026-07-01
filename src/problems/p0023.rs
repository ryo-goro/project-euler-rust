use crate::math::divisors;

const LIMIT: u64 = 28123;

pub fn solve() -> u64 {
    let mut abundants = Vec::new();

    for n in 2..=LIMIT {
        if is_abundant(n) {
            abundants.push(n);
        }
    }

    let mut is_abundant_sum = [false; (LIMIT + 1) as usize];

    'outer: for (i, &a) in abundants.iter().enumerate() {
        for &b in &abundants[i..] {
            let sum = a + b;

            if sum > LIMIT {
                if a == b {
                    // No larger abundant number can produce a valid sum.
                    break 'outer;
                }
                break;
            }

            is_abundant_sum[sum as usize] = true;
        }
    }

    is_abundant_sum
        .iter()
        .enumerate()
        .filter_map(|(n, is_sum)| (!is_sum).then_some(n as u64))
        .sum()
}

fn is_abundant(n: u64) -> bool {
    divisors::sum_of_proper_divisors(n) > n
}
