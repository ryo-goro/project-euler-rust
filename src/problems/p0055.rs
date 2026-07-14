use crate::math::digits;
use crate::math::palindrome;

const LIMIT: u64 = 10000;
const ITERATION_LIMIT: usize = 50;
const MAX_DIGITS: usize = 64;

pub fn solve() -> i64 {
    let mut count = 0;
    let mut digits = [0; MAX_DIGITS];

    'outer: for x in 1..LIMIT {
        let mut num_digits = to_digits(x, &mut digits);

        for _ in 1..ITERATION_LIMIT {
            num_digits = reverse_and_add(&mut digits, num_digits);

            if palindrome::is_palindrome(&digits[..num_digits]) {
                continue 'outer;
            }
        }

        count += 1;
    }

    count
}

fn to_digits(mut x: u64, digits: &mut [u64]) -> usize {
    let mut len = 0;

    loop {
        digits[len] = x % 10;

        x /= 10;
        len += 1;

        if x == 0 {
            break;
        }
    }

    len
}

fn reverse_and_add(digits: &mut [u64], num_digits: usize) -> usize {
    let mut rev = [0; MAX_DIGITS];
    
    for i in 0..num_digits {
        rev[i] = digits[num_digits - 1 - i];
    }

    for i in 0..num_digits {
        digits[i] += rev[i];
    }

    digits::propagate_carry(digits, num_digits)
}
