use std::collections::HashSet;

pub fn solve() -> i64 {
    let mut products = HashSet::new();

    // The possible combinations are:
    // - 1-digit * 4-digit = 4-digit
    // - 2-digit * 3-digit = 4-digit

    // 1-digit * 4-digit = 4-digit
    for a in 1..10 {
        for b in 1000..10000 {
            let c = a * b;

            if c >= 10000 {
                break;
            }

            if is_pandigital(a, b, c) {
                products.insert(c);
            }
        }
    }

    // 2-digit * 3-digit = 4-digit
    for a in 10..100 {
        for b in 100..1000 {
            let c = a * b;

            if c >= 10000 {
                break;
            }

            if is_pandigital(a, b, c) {
                products.insert(c);
            }
        }
    }
    
    products.iter().sum::<u64>() as i64
}

fn is_pandigital(a: u64, b: u64, c: u64) -> bool {
    // Bit mask implementation.
    
    let mut used = 0u16;

    for mut n in [a, b, c] {
        if n == 0 {
            return false;
        }

        while n > 0 {
            let digit = (n % 10) as u16;

            if digit == 0 || used & (1 << digit) != 0 {
                return false;
            }

            used |= 1 << digit;
            n /= 10;
        }
    }

    used == 0b11_1111_1110  // Expected bit mask for a 1–9 pandigital number.
}
