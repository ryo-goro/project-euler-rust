// Since `usize` is at most 64 bits, and `2^64 - 1 < 21!`,
// the largest factoradic representation of a `usize` has 21 digits.
const MAX_FACTORADIC_DIGITS: usize = 21;

/// Returns the `n`th permutation of `perm_len`s integers,
/// `0`, `1`, ..., `perm_len - 1`, in lexicographic order.
/// 
/// If `n >= perm_len!`, it wraps around modulo `perm_len!`.
///
/// Like most indexing operations, the count starts from zero, so `nth_permutation(perm_len, 0)`
/// returns the first permutation, `nth_permutation(perm_len, 1)` the second, and so on.
pub fn nth_permutation(perm_len: usize, mut n: usize) -> Vec<usize> {
    // `n` is a `usize`, so `n <= usize::MAX < 21!` on supported platforms.
    // `u8` because each factoradic digit is at most 20.
    let mut factoradic_digits = [0u8; MAX_FACTORADIC_DIGITS];
    let mut digits_len = perm_len;

    // Extract the factoradic digits.
    // Since `n < 21!`, this loop updates at most 21 digits.
    for k in 0..perm_len {
        if n == 0 {
            digits_len = k;
            break;
        }

        factoradic_digits[k] = (n % (k + 1)) as u8;
        n /= k + 1;
    }

    let mut permutation = Vec::with_capacity(perm_len);

    // Elements smaller than `offset` are fixed because the omitted
    // leading factoradic digits are implicitly zero.
    let offset = perm_len - digits_len;
    permutation.extend(0..offset);

    // Bit mask implementation.
    // u32 is big enough because 32 > MAX_FACTORADIC_DIGITS.
    let mut used = 0u32;

    for &digit in factoradic_digits[..digits_len].iter().rev() {
        let mut digit = digit;

        // Find the `digit`-th unused element in ascending order.
        for element in 0..digits_len {
            if used & (1 <<element) != 0 {
                continue;
            }

            if digit == 0 {
                permutation.push(element + offset);
                used |= 1 << element;
                break;
            }

            digit -= 1;
        }
    }

    permutation
}
