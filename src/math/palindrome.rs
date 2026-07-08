pub fn is_palindrome<T: PartialEq>(slice: &[T]) -> bool {
    slice.iter().eq(slice.iter().rev())
}

pub fn is_palindromic_base(mut x: u64, base: u8) -> bool {
    assert!(base >= 2);

    // Maximum digits needed for a u64 in base 2.
    let mut buf = [0u8; 64];
    let mut len = 0;
    let base = base as u64;

    while x > 0 {
        buf[len] = (x % base) as u8;
        x /= base;
        len += 1;
    }

    is_palindrome(&buf[..len])
}

pub fn is_palindromic(x: u64) -> bool {
    is_palindromic_base(x, 10)
}
