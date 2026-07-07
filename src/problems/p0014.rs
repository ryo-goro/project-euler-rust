const LIMIT: usize = 1000000;

// The cache size is independent of LIMIT,
// but using the same value is a reasonable trade-off.
const CACHE_LEN: usize = LIMIT;

pub fn solve() -> i64 {
    // Use a Vec to avoid stack overflow during tests.
    let mut cache = vec![0; CACHE_LEN];
    cache[1] = 1;

    let mut answer = 1;
    let mut max_length = 1;

    for n in 2..LIMIT {
        let length = collatz_length(n as u64, &mut cache);

        if length > max_length {
            max_length = length;
            answer = n as i64;
        }
    }

    answer
}

// Traverse until reaching a cached value,
// then propagate the computed lengths back through the path.
fn collatz_length(mut n: u64, cache: &mut [usize]) -> usize {
    let cache_limit = cache.len() as u64;
    let mut path = Vec::with_capacity(256);

    while n >= cache_limit || cache[n as usize] == 0 {
        path.push(n);

        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    let mut length = cache[n as usize];

    while let Some(x) = path.pop() {
        length += 1;

        if x < cache_limit {
            cache[x as usize] = length;
        }
    }

    length
}
