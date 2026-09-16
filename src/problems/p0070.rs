const N: usize = 10_000_000;

pub fn solve() -> i64 {
    let totients = list_totients(N);

    let mut res = 2;

    for (n, &totient) in totients.iter().enumerate().skip(3) {
        if has_same_digits(n, totient)
            && n * totients[res] < res * totient    // n / φ(n) < res / φ(res)
        {
            res = n;
        }
    }

    res as i64
}

fn list_totients(size: usize) -> Vec<usize> {
    let mut totients: Vec<usize> = (0..size).collect();

    for n in 2..size {
        if totients[n] == n {
            // `n` is prime.
            for i in (n..size).step_by(n) {
                totients[i] -= totients[i] / n
            }
        }
    }

    totients
}

fn count_digits(mut n: usize) -> [u8; 10] {
    let mut counts = [0u8; 10];

    if n == 0 {
        counts[0] = 1;
        return counts;
    }

    while n > 0 {
        counts[n % 10] += 1;
        n /= 10;
    }

    counts
}

fn has_same_digits(a: usize, b: usize) -> bool {
    // Let `a` and `b` be:
    // a = a_{0} + 10 * a_{1} + ... + 10^{m} * a_{m},
    // b = b_{0} + 10 * b_{1} + ... + 10^{n} * b_{n}.
    //
    // If both `a` and `b` have the same digits, then
    // (a_{0} + a_{1} + ... + a_{m}) % 9 = (b_{0} + b_{1} + ... + b_{n}) % 9.
    // 
    // So `a % 9 == b % 9` holds.
    // This is logically not necessary, but much cheaper than calling `count_digits()`.

    a % 9 == b % 9 && count_digits(a) == count_digits(b)
}
