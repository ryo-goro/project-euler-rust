const ONES: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const TEENS: [&str; 10] = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TENS: [&str; 8] = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

pub fn solve() -> u64 {
    let ones_sum: usize = ONES.iter().map(|one| one.len()).sum();       // 1 ~ 9
    let teens_sum: usize = TEENS.iter().map(|teen| teen.len()).sum();   // 10 ~ 19
    let tens_sum: usize = TENS.iter().map(|ten| ten.len()).sum();       // 20, 30, ..., 90

    // 1 ~ 99
    let sum_below_100 =
        ones_sum                          // 1 ~ 9
        + teens_sum                       // 10 ~ 19
        // Each tens word appears 10 times, and each one-digit word appears with 8 tens prefixes.
        + tens_sum * 10 + ones_sum * 8;   // 20 ~ 99
    
    let total_letters =
        ones_sum * 100                      // Letters contributed by "one" to "nine" in "one hundred" through "nine hundred".
        + "hundred".len() * 100 * 9
        + "and".len() * 99 * 9
        + sum_below_100 * 10                // The suffixes "one" to "ninety-nine" are repeated for each hundred block.
        + "one".len() + "thousand".len();
    
    total_letters as u64
}
