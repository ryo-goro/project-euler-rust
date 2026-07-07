use crate::math::gcd;

const N: usize = 20;

pub fn solve() -> i64 {
    // Compute C(40, 20) by cancelling common factors before multiplication:
    // C(40, 20) = 40! / (20!)^2 = (40 * ... * 21) / (20 * ... * 1)
    let mut numerators: Vec<u64> = (N + 1..=2 * N).map(|n| n as u64).collect();

    // Cancel each denominator against the remaining numerators.
    for denominator in 2..=N {
        let mut denominator = denominator as u64;

        for numerator in &mut numerators {
            let common = gcd::gcd(denominator, *numerator);

            denominator /= common;
            *numerator /= common;

            if denominator == 1 {
                break;
            }
        }
    }

    numerators.iter().product::<u64>() as i64
}
