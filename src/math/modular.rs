/// Computes `(base^exp) mod modulo` using binary exponentiation.
pub fn pow_mod(base: u64, mut exp: u64, modulo: u64) -> u64 {
    assert!(modulo > 0);

    if modulo == 1 {
        return 0;
    }

    let mut base = (base % modulo) as u128;
    let modulo = modulo as u128;
    let mut result = 1u128;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulo;
        }

        base = base * base % modulo;
        exp >>= 1;
    }

    result as u64
}
