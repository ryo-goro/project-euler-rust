use crate::math::palindrome;

const LIMIT: u64 = 1_000_000;

pub fn solve() -> i64 {
    (1..LIMIT)
        .filter(|&n| {
            palindrome::is_palindromic(n) 
            && palindrome::is_palindromic_base(n, 2)
        })
        .sum::<u64>() as i64
}
