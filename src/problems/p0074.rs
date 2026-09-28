const LIMIT: usize = 1_000_000;
const MAX_CHAIN_LEN: u8 = 60;

// 0! ~ 9!
const FACTORIALS: [usize; 10] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880,
];

pub fn solve() -> i64 {
    let mut lengths = [0u8; LIMIT];

    // Known loops.
    // 169 -> 363601 -> 1454 -> 169
    lengths[169] = 3;
    lengths[363601] = 3;
    lengths[1454] = 3;

    // 871 -> 45361 -> 871
    lengths[871] = 2;
    lengths[45361] = 2;

    // 872 -> 45362 -> 872
    lengths[872] = 2;
    lengths[45362] = 2;

    let mut chain = Vec::with_capacity(MAX_CHAIN_LEN as usize);
    let mut count = 0;

    for n in 1..LIMIT {
        chain.clear();

        let mut term = n;

        let trailing_len = loop {
            if term < LIMIT && lengths[term] > 0 {
                break lengths[term];
            }

            let next = sum_digit_factorials(term);

            if term == next {
                if term < LIMIT {
                    lengths[term] = 1;
                }
                
                break 1;
            }

            chain.push(term);
            term = next;
        };

        let chain_len = chain.len() as u8 + trailing_len;

        if chain_len == MAX_CHAIN_LEN {
            count += 1;
        }

        for (i, &term) in chain.iter().enumerate() {
            if term < LIMIT {
                lengths[term] = chain_len - i as u8;
            }
        }
    }

    count
}

fn sum_digit_factorials(mut n: usize) -> usize {
    if n == 0 {
        // 0! = 1
        return 1;
    }

    let mut sum = 0;

    while n > 0 {
        sum += FACTORIALS[n % 10];
        n /= 10;
    }

    sum
}
