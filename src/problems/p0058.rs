use crate::math::primes;

pub fn solve() -> i64 {
    // 37 36 35 34 33 32 31
    // 38 17 16 15 14 13 30
    // 39 18  5  4  3 12 29
    // 40 19  6  1  2 11 28
    // 41 20  7  8  9 10 27
    // 42 21 22 23 24 25 26
    // 43 44 45 46 47 48 49

    // To upper right:  1, 3, 13, 31, ... -> 4n^2 - 10n + 7
    // To upper left:   1, 5, 17, 37, ... -> 4n^2 -  8n + 5
    // To bottom left:  1, 7, 21, 43, ... -> 4n^2 -  6n + 3
    // To bottom right: 1, 9, 25, 49, ... -> 4n^2 -  4n + 1 = (2n - 1)^2

    // Clearly the bottom right numbers are not prime.

    let mut layer = 1u64;
    let mut num_diagonals = 1;
    let mut num_primes = 0;

    loop {
        layer += 1;
        num_diagonals += 4;

        // Keep additions before subtractions to avoid u64 underflow for layer=2.
        let corners = [
            4 * layer * layer + 7 - 10 * layer, // upper right
            4 * layer * layer + 5 - 8 * layer,  // upper left
            4 * layer * layer + 3 - 6 * layer,  // bottom left
        ];

        num_primes += corners
            .iter()
            .filter(|&&x| primes::is_prime(x))
            .count();

        // Check if the ratio of primes along both diagonals falls below 10%.
        if 10 * num_primes < num_diagonals {
            break;
        }
    }

    (2 * layer - 1) as i64
}
