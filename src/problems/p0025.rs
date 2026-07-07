use crate::math::digits;

const MAX_DIGITS: usize = 1000;

pub fn solve() -> i64 {
    let mut buf1 = [0; MAX_DIGITS];
    let mut buf2 = [0; MAX_DIGITS];
    buf1[0] = 1;
    buf2[0] = 1;

    let mut prev_len = 1;
    let mut curr_len = 1;

    let mut prev = &mut buf1;
    let mut curr = &mut buf2;

    let mut n = 2;

    while curr_len < MAX_DIGITS {
        n += 1;

        add_assign_digits(prev, &curr[..curr_len]);

        // curr is never shorter than prev.
        prev_len = digits::propagate_carry(prev, curr_len);
        
        std::mem::swap(&mut prev, &mut curr);
        std::mem::swap(&mut prev_len, &mut curr_len);
    }

    n
}

fn add_assign_digits(augend: &mut [u64], addend: &[u64]) {
    assert!(augend.len() >= addend.len());

    for (i, &digit) in addend.iter().enumerate() {
        augend[i] += digit;
    }
}
