pub fn is_prime(x: u64) -> bool {
    if x <= 3 {
        return x >= 2;
    }

    if x.is_multiple_of(2) {
        return false;
    }

    let mut d = 3;

    while d <= x / d {
        if x.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }

    true
}
