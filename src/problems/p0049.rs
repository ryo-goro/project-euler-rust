use crate::math::primes;
use crate::math::permutation;

const NUM_DIGITS: usize = 4;
const LIMIT: u64 = 10u64.pow(NUM_DIGITS as u32);

pub fn solve() -> i64 {
    let is_prime = primes::sieve(LIMIT as usize);

    // Mark every generated permutation as processed so that each digit multiset
    // is handled only once.
    // Not necessary, just for speed.
    let mut checked = [false; LIMIT as usize];

    let mut digits = [0; NUM_DIGITS];

    // At most `NUM_DIGITS`! distinct permutations.
    let mut candidates = [0; 24];

    for mut n in LIMIT / 10..LIMIT {
        if checked[n as usize] {
            continue;
        }

        checked[n as usize] = true;

        let mut num_candidates = 0;

        if is_prime[n as usize] {
            candidates[num_candidates] = n;
            num_candidates += 1;
        }

        for digit in digits.iter_mut().rev() {
            *digit = n % 10;
            n /= 10;
        }

        while permutation::next_permutation(&mut digits) {
            let candidate = digits.iter().fold(0, |num, &digit| 10 * num + digit);

            checked[candidate as usize] = true;

            if is_prime[candidate as usize] {
                candidates[num_candidates] = candidate;
                num_candidates += 1;
            }
        }

        if num_candidates < 3 {
            continue;
        }

        // `candidates` is sorted in ascending order because permutations are
        // generated in lexicographic order.

        for (i, &first) in candidates[..num_candidates - 2].iter().enumerate() {
            for (offset, &second) in candidates[i + 1..num_candidates - 1].iter().enumerate() {
                let j = i + 1 + offset;

                let diff = second - first;
                let third = second + diff;

                if candidates[j + 1..num_candidates].binary_search(&third).is_ok()
                    // We need to find a sequence other than (1487, 4817, 8147).
                    && (first, second, third) != (1487, 4817, 8147)
                {
                    return ((first * LIMIT + second) * LIMIT + third) as i64;
                }
            }
        }
    }

    unreachable!("answer could not be found");
}
