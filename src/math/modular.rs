/// Computes `(base^exp) mod modulo` using binary exponentiation.
pub fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    assert!(modulus > 0);

    if modulus == 1 {
        return 0;
    }

    let mut base = base % modulus;
    let mut result = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }

        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }

    result
}

/// Computes `(a * b) mod modulus` without overflowing.
pub fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}
