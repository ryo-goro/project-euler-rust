use std::cmp::Ordering;

use crate::math::primes;

const PRIME_LIMIT: u64 = 1_000_000;
const SET_SIZE: usize = 5;
const STACK_SIZE: usize = SET_SIZE - 1;

pub fn solve() -> i64 {
    let sieve = primes::sieve(PRIME_LIMIT as usize);

    // [3, 7, 13, 19, 31, ...]
    let primes_6k_plus_1 = std::iter::once(3)
        .chain(
            sieve
                .iter()
                .enumerate()
                .filter_map(|(p, &is_prime)| (is_prime && p % 6 == 1).then_some(p as u64))
        )
        .collect();

    // [3, 5, 11, 17, 23, ...]
    let primes_6k_plus_5 = std::iter::once(3)
        .chain(
            sieve
                .iter()
                .enumerate()
                .filter_map(|(p, &is_prime)| (is_prime && p % 6 == 5).then_some(p as u64))
        )
        .collect();

    let mut pa_6k_plus_1 = PrimeAdjacency::new(primes_6k_plus_1);
    let mut pa_6k_plus_5 = PrimeAdjacency::new(primes_6k_plus_5);
    
    let mut best: Option<u64> = None;

    let mut prime_limit = 1000;

    let result = loop {
        if prime_limit > PRIME_LIMIT {
            panic!(
                "prime_limit ({prime_limit}) has exceeded PRIME_LIMIT ({PRIME_LIMIT}). \
                No solution was found within the sieve range — \
                increase PRIME_LIMIT and try again."
            );
        }

        let res = search(prime_limit, best, &mut pa_6k_plus_1);
        update_min(&mut best, res);

        let res = search(prime_limit, best, &mut pa_6k_plus_5);
        update_min(&mut best, res);
        
        if let Some(current) = best && current < prime_limit {
            break current;
        }

        prime_limit *= 2;
    };

    result as i64
}

struct PrimeAdjacency {
    primes: Vec<u64>,
    neighbors: Vec<Vec<usize>>,
    computed_upto: Vec<usize>,
}

impl PrimeAdjacency {
    fn new(primes: Vec<u64>) -> Self {
        let len = primes.len();

        Self {
            primes,
            neighbors: vec![Vec::new(); len],
            computed_upto: (1..=len).collect(),
        }
    }

    fn ensure_computed(&mut self, i: usize, upto: usize) {
        assert!(i < self.primes.len());

        let start = self.computed_upto[i];

        if start >= upto {
            return;
        }

        let x = self.primes[i];

        self.neighbors[i].extend(
            self.primes[start..upto]
                .iter()
                .enumerate()
                .filter_map(|(idx, &p)|
                    (
                        primes::is_prime(concat(x, p))
                        && primes::is_prime(concat(p, x))
                    )
                    .then_some(start + idx)
                )
        );

        self.computed_upto[i] = upto;
    }
}

fn update_min(best: &mut Option<u64>, candidate: Option<u64>) {
    if let Some(x) = candidate && best.is_none_or(|b| x < b) {
        *best = Some(x);
    }
}

fn concat(a: u64, b: u64) -> u64 {
    let mut pow10 = 10;

    // Find the smallest power of 10 greater than b.
    while pow10 <= b {
        pow10 *= 10;
    }

    a * pow10 + b
}

// `a` should be sorted in strictly descending order.
// `b` should be sorted in strictly ascending order.
// Returns the intersection of `a` and `b` in strictly descending order.
fn intersection(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;

    let mut res = Vec::new();

    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[b.len() - 1 - j]) {
            Ordering::Less => j += 1,
            Ordering::Greater => i += 1,
            Ordering::Equal => {
                res.push(a[i]);
                i += 1;
                j += 1;
            },
        }
    }

    res
}

struct DfsFrame {
    candidates: Vec<usize>,
    sum: u64,
}

fn search(prime_limit: u64, best: Option<u64>, pa: &mut PrimeAdjacency) -> Option<u64> {
    let mut best = best.unwrap_or(u64::MAX);
    let num_primes = pa.primes.partition_point(|&p| p < prime_limit);
    let mut dfs_stk = Vec::with_capacity(STACK_SIZE);

    for idx in 0..num_primes {
        if pa.primes[idx] * SET_SIZE as u64 >= best {
            break;
        }

        pa.ensure_computed(idx, num_primes);

        let candidates: Vec<usize> = pa.neighbors[idx]
            .iter()
            .rev()
            .copied()
            .collect();

        dfs_stk.push(DfsFrame {
            candidates,
            sum: pa.primes[idx],
        });

        while let Some(mut frame) = dfs_stk.pop() {
            let Some(next_idx) = frame.candidates.pop() else {
                continue;
            };

            let next_prime = pa.primes[next_idx];
            let num_selected = dfs_stk.len() + 1;
            let lower_bound = frame.sum + ((SET_SIZE - num_selected) as u64) * next_prime; 

            if lower_bound >= best {
                continue;
            };

            let next_sum = frame.sum + next_prime;
            dfs_stk.push(frame);

            if dfs_stk.len() == STACK_SIZE {
                best = best.min(next_sum);
                continue;
            }

            pa.ensure_computed(next_idx, num_primes);

            let next_candidates = intersection(&dfs_stk.last().unwrap().candidates, &pa.neighbors[next_idx]);

            dfs_stk.push(DfsFrame {
                candidates: next_candidates,
                sum: next_sum,
            });
        }
    }

    (best < u64::MAX).then_some(best)
}
