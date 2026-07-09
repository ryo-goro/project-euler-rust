pub fn solve() -> i64 {
    let perimeter: u64 = 1000;

    // 0 < a < b < c, a + b + c = perimeter, hence 1 <= a < perimeter / 3
    for a in 1..perimeter / 3 {
        // a < b < c, a + b + c = perimeter, hence a + 1 <= b < (perimeter - a + 1) / 2
        for b in a + 1..(perimeter - a + 1) / 2 {
            let c = perimeter - a - b;

            if a * a + b * b == c * c {
                return (a * b * c) as i64;
            }
        }
    }

    unreachable!("there exists exactly one Pythagorean triplet for which a + b + c = 1000");
}
