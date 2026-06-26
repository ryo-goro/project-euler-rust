use crate::math::palindrome;

pub fn solve() -> u64 {
    let mut answer = 0;

    for a in 100..1000 {
        for b in 100..1000 {
            let p = a * b;

            if palindrome::is_palindromic(p) && p > answer {
                answer = p;
            }
        }
    }

    answer
}
