pub fn solve() -> i64 {
    (0..7)
        .map(|n| nth_champernowne_digit(10usize.pow(n) - 1))
        .product::<u64>() as i64
}

// Returns the `n`th digit of `x`.
// Zero-based indexing.
// Example: nth_digit(123, 0) = 3, nth_digit(123, 1) = 2, nth_digit(123, 2) = 1. 
// Example: nth_digit(123, 4) = nth_digit(00123, 4) = 0.
fn nth_digit(mut x: u64, n: usize) -> u64 {
    for _ in 0..n {
        x /= 10;

        if x == 0 {
            return 0;
        }
    }

    x % 10
}

// Returns the `n`th digit of the fractional part of the Champernowne constant
// Zero-based indexing.
fn nth_champernowne_digit(n: usize) -> u64 {
    // Champernowne constant is 0.|123456789|1011...99|100101...999|1000...
    // Let's call the sequence 123456789 the first block, 1011...99 the second block, etc.,
    // and the `n`th digit (`n` >= 0) in the fractional part of the Champernowne constant "the target digit".

    // If the target digit is in the `m`th block (`m` >= 0), `n` and `m` fulfill `s_m <= n < s_{m + 1}`, and vice versa,
    // where `s_k` (`k` >= 0) is the index of the first digit of the `k`th block.

    // Number of digits per number in the current block.
    let mut num_digits = 1;

    // Total number of digits in the current block.
    let mut total_digits = 9;

    // Index of the first digit in the current block.
    let mut first_index = 0;

    // First number in the current block, i.e., 10^{num_digits - 1}.
    let mut first_num = 1u64;

    while first_index + total_digits <= n {
        first_index += total_digits;
        num_digits += 1;
        first_num *= 10;

        // The current block contains `9 * num_digits * 10^{num_digits - 1}` digits.
        total_digits = 9 * num_digits * first_num as usize;
    }
    
    // `target` is the number in the current block that contains the target digit.
    // Example: If n = 9 or 10, target = 10.
    // Example: If n = 189, 190 or 191, target = 100.
    let offset = n - first_index;
    let target = first_num + (offset / num_digits) as u64;
    let pos = num_digits - 1 - offset % num_digits;

    nth_digit(target, pos)
}
