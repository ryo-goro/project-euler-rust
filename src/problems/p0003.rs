pub fn solve() -> u64 {
    let mut target = 600851475143u64;
    let mut divisor = 3;
    let mut answer = 0;

    while divisor * divisor <= target {
        let mut tmp = target;

        while tmp.is_multiple_of(divisor) {
            tmp /= divisor;
        }

        if tmp != target {
            target = tmp;
            answer = divisor;
        }

        divisor += 2;
    }

    if answer < target {
        answer = target;
    }

    answer
}
