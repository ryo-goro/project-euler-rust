const MAX_DENOMINATOR: usize = 1_000_000;

pub fn solve() -> i64 {
    // The number of reduced proper fractions whose denominators are d > 1 is φ(d).
    
    let totients = list_totients(MAX_DENOMINATOR + 1);

    totients[2..]
        .iter()
        .map(|&t| t as i64)
        .sum()
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
