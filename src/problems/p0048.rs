use crate::math::modular;

const NUM_DIGITS: u32 = 10;
const UPPER_BOUND: u64 = 1000;

pub fn solve() -> i64 {

    let mut sum = 0;
    let modulo = 10u64.pow(NUM_DIGITS);

    for n in 1..=UPPER_BOUND {
        sum = (sum + modular::mod_pow(n, n, modulo)) % modulo;
    }
    
    sum as i64
}
