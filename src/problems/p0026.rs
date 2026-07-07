const LIMIT: usize = 1000;

pub fn solve() -> u64 {
    let mut max_len = 0;
    let mut res = 1;

    for n in 2..LIMIT {
        let len = cycle_len(n);

        if len > max_len {
            max_len = len;
            res = n;
        }
    }

    res as u64
}

fn cycle_len(n: usize) -> usize {
    assert!(n > 0);
    
    let mut first_seen = vec![0; n];
    let mut rem = 10 % n;
    let mut pos = 1;

    while rem != 0 {
        if first_seen[rem] != 0 {
            return pos - first_seen[rem];
        }

        first_seen[rem] = pos;
        rem = rem * 10 % n;
        pos += 1;
    }

    0
}
