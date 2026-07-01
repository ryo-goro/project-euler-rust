pub fn propagate_carry(digits: &mut [u64], num_digits: usize) -> usize {
    assert!(num_digits > 0);

    let mut highest = num_digits - 1;

    for i in 0..highest {
        digits[i + 1] += digits[i] / 10;
        digits[i] %= 10;
    }

    while digits[highest] >= 10 {
        digits[highest + 1] = digits[highest] / 10;
        digits[highest] %= 10;
        highest += 1;
    }

    highest + 1
}
