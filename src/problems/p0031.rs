const TARGET_AMOUNT: usize = 200;
const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

pub fn solve() -> i64 {
    let mut dp = [0; TARGET_AMOUNT + 1];
    dp[0] = 1;

    // After processing each coin, dp[amount] stores the number of ways
    // to make `amount` using only the coins processed so far.
    for coin in COINS {
        for amount in coin..=TARGET_AMOUNT {
            dp[amount] += dp[amount - coin];
        }
    }

    dp[TARGET_AMOUNT] as i64
}
