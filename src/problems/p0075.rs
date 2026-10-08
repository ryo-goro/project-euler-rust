use crate::math::gcd;

const MAX_HALF_PERIMETER: u64 = 1_500_000 / 2;

// Every primitive Pythagorean triple (a, b, c) with a even can be written
// uniquely as a = 2mn, b = m^2 - n^2, c = m^2 + n^2 for some m > n > 0
// with (i) opposite parity and (ii) gcd(m, n) = 1.
// See docs/math/topics/pythagorean_triples.md
// for the derivation and proof of this parametrization.
pub fn solve() -> i64 {
    let mut counts = vec![0; (MAX_HALF_PERIMETER + 1) as usize];
    
    for m in 1..=MAX_HALF_PERIMETER.isqrt() {
        // `m` and `n` must have opposite parity for primitive Pythagorean triples.
        for n in (m % 2 + 1..m).step_by(2) {
            // Half-perimeter of the primitive triple (a, b, c) = (2mn, m^2 - n^2, m^2 + n^2):
            // (a + b + c) / 2 = m * (m + n).
            let primitive_half_perimeter = m * (m + n);

            if primitive_half_perimeter > MAX_HALF_PERIMETER {
                break;
            }

            // `m` and `n` must be coprime for primitive Pythagorean triples.
            if gcd::gcd(m, n) == 1 {
                // Every (not necessarily primitive) Pythagorean triple is
                // an integer multiple `k` of some primitive triple,
                // and its half-perimeter is `k * primitive_half_perimeter`.
                // So we tally one triple for every multiple of `primitive_half_perimeter`
                // up to `MAX_HALF_PERIMETER`.
                for half_perimeter in
                    (primitive_half_perimeter..=MAX_HALF_PERIMETER)
                        .step_by(primitive_half_perimeter as usize)
                {
                    counts[half_perimeter as usize] += 1;
                }
            }
        }
    }

    counts
        .into_iter()
        .filter(|&count| count == 1)
        .count() as i64
}
