use std::mem;
use crate::math::biguint::BigUint;

const MAX_ITERATIONS: usize = 1000;

pub fn solve() -> i64 {
    // Let {a_n} be a sequence defined by the following recurrence relation:
    //
    // - a_0 = 1
    // - a_(n + 1) = 1 + 1 / (a_n + 1) (n >= 0)
    // 
    // Then a_n (n >= 1) is the n-th expansion of the infinite continued fraction of the square root of 2
    //
    // Let {p_n} and {q_n} be natural number sequences defined by the following recurrence relations:
    // 
    // - p_0 = q_0 = 1
    // - p_(n + 1) = p_n + q_n (n >= 0)
    // - q_(n + 1) = 2 * p_n + q_n (= p_n + p_(n + 1)) (n >= 0)
    // 
    // Then the following properties hold:
    // 
    // 1. p_n and q_n are coprime (n >= 0)
    // 2. a_n = q_n / p_n (n >= 0)

    let mut numerator = BigUint::one();
    let mut denominator = BigUint::one();

    let mut count = 0;

    for _ in 0..MAX_ITERATIONS {
        // Swap first so that
        // denominator = q_n
        // numerator   = p_n
        mem::swap(&mut numerator, &mut denominator);

        // denominator = q_n + p_n = p_(n + 1)
        denominator += &numerator;

        // numerator = p_n + p_(n + 1) = q_(n + 1)
        numerator += &denominator;

        if numerator.num_digits() > denominator.num_digits() {
            count += 1;
        }
    }

    count
}
