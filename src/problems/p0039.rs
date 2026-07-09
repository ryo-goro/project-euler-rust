const MAX_PERIMETER: u64 = 1000;

pub fn solve() -> i64 {
    let mut res = 0;
    let mut max_count = 0;

    for p in (6..=MAX_PERIMETER).step_by(2) {
        let mut count = 0;

        for a in 1..p / 3 {
            for b in a + 1..(p - a + 1) / 2 {
                let c = p - a - b;

                if a * a + b * b == c * c {
                    count += 1;
                }
            }
        }

        if count > max_count {
            max_count = count;
            res = p;
        }
    }

    res as i64
}
