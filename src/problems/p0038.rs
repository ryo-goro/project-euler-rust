// Concatenated product of 9 and (1, 2, 3, 4, 5).
// The result must be equal to or more than this value.
const LOWER_BOUND: u64 = 918273645;

pub fn solve() -> i64 {
    let mut res = LOWER_BOUND;

    // If the concatenated product of x and (1, 2, ..., n) (n > 1) is the maximum value:
    // - The most significant digit of x is 9.
    // - x has at most 4 digits.
    // - x does not have 0 as any of its digits.
    for right in 1..1000 {
        // Example: right = 123 -> x = 9123
        let x = concat(9, right);
        let mut candidate = x;

        for n in 2.. {
            candidate = concat(candidate, x * n);
            if num_digits(candidate) >= 9 {
                break;
            } 
        }

        if candidate > res && is_pandigital(candidate) {
            res = candidate;
        }
    }

    res as i64
}

fn concat(a: u64, b: u64) -> u64 {
    let mut pow10 = 10;

    // Find the smallest power of 10 greater than b.
    while pow10 <= b {
        pow10 *= 10;
    }

    a * pow10 + b
}

fn num_digits(mut x: u64) -> usize {
    let mut count = 1;

    while x >= 10 {
        x /= 10;
        count += 1;
    }

    count
}

fn is_pandigital(mut x: u64) -> bool {
    // Bit mask implementation.
    
    let mut used = 0u16;

    while x > 0 {
        let digit = (x % 10) as u16;

        if digit == 0 || used & (1 << digit) != 0 {
            return false;
        }

        used |= 1 << digit;
        x /= 10;
    }

    used == 0b11_1111_1110  // Expected bit mask for a 1–9 pandigital number.
}
