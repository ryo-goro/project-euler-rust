pub fn solve() -> i64 {
    let mut target = 600851475143;
    let mut divisor = 3;
    let mut answer = 0;

    while divisor * divisor <= target {
        let mut tmp = target;

        while tmp % divisor == 0 {
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
