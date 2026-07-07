use crate::math::gcd;

pub fn solve() -> i64 {
    let mut numerator_product = 1;
    let mut denominator_product = 1;

    // The target fraction is numerator / denominator where 10 <= numerator < denominator < 100.
    // Let numerator = 10a + b, denominator = 10c + d (b, d != 0),
    // where a, c are the tens digits and b, d are the ones digits.
    // Then the only non-trivial cases are:
    // - b = c, and numerator / denominator = (10a + b) / (10b + d) = a / d.
    // - a = d, and numerator / denominator = (10a + b) / (10c + a) = b / c.
    for numerator in 10..100 {
        let b = numerator % 10;

        if b == 0 {
            continue;
        }

        let a = numerator / 10;

        for denominator in numerator + 1..100 {
            let d = denominator % 10;

            if d == 0 {
                continue;
            }

            let c = denominator / 10;

            if (b == c && numerator * d == denominator * a)
                || (a == d && numerator * c == denominator * b)
            {
                numerator_product *= numerator;
                denominator_product *= denominator;
            }
        }
    }

    (denominator_product / gcd::gcd(numerator_product, denominator_product)) as i64
}
