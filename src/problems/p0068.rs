use crate::math::permutation;

const N: usize = 5;

pub fn solve() -> i64 {
    //        o[0]
    //         \
    //          \          o[1]
    //           i[0]     /
    //        /        \ /
    //       i[4]       i[1]
    //      / \        /
    //     /   i[3] - i[2] -- o[2]
    //    o[4]  \
    //           \
    //            o[3]
    //
    // o[k]: outer[k]
    // i[k]: inner[k]
    //
    // Line `k` consists of `outer[k]`, `inner[k]`, and `inner[(k + 1) % N]`.
    //
    // To obtain a 16-digit result, 10 must be an outer node.
    // Due to rotational symmetry, fix it at `outer[0]`.

    let mut digits: [u64; 2 * N] = std::array::from_fn(|n| n as u64);
    digits[0] = 10;

    let mut res = 0;

    loop {
        let (outer, inner) = digits.split_at(N);

        if is_magical(outer, inner) {
            res = res.max(assemble(outer, inner));
        }

        if !permutation::next_permutation(&mut digits[1..]) {
            break;
        }
    }

    res as i64
}

fn is_magical(outer: &[u64], inner: &[u64]) -> bool {
    // Use the final line as the reference sum.
    let sum = outer[N - 1] + inner[N - 1] + inner[0];

    for i in 0..N - 1 {
        if outer[i] + inner[i] + inner[i + 1] != sum {
            return false;
        }
    }

    true
}

fn assemble(outer: &[u64], inner: &[u64]) -> u64 {
    let mut res = 0;

    // Start from the line with the numerically lowest outer node.
    let start = (0..N).min_by_key(|&i| outer[i]).unwrap();

    for offset in 0..N {
        let idx = (start + offset) % N;
        let next = (idx + 1) % N;

        let line_val = outer[idx] * 100 + inner[idx] * 10 + inner[next];

        // The group containing 10 has four digits; all others have three.
        res *= if outer[idx] == 10 { 10000 } else { 1000 };
        res += line_val;
    }

    res
}
