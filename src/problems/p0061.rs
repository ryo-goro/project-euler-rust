use crate::math::permutation;

const NUM_POLYGONAL_KINDS: usize = 6;
const NUM_DIGITS: usize = 4;
const LINK_DIGITS: usize = 2;
const PREFIX_DIVISOR: u64 = 10u64.pow((NUM_DIGITS - LINK_DIGITS) as u32);
const SUFFIX_MODULUS: u64 = 10u64.pow(LINK_DIGITS as u32);

struct DfsFrame {
    kind_offset: usize,
    cursor: usize,
    end: usize,
}

pub fn solve() -> i64 {
    let polygonal_numbers_list: Vec<Vec<u64>> = {
        let begin = 10u64.pow((NUM_DIGITS - 1) as u32);
        let end = begin * 10;

        (0..NUM_POLYGONAL_KINDS)
            .map(|offset| polygonal_numbers(offset, begin, end))
            .collect()
    };

    // Fix the last kind (octagonal) and permute the remaining kinds.
    let mut kind_offsets: [usize; NUM_POLYGONAL_KINDS - 1] = std::array::from_fn(|i| i);
    let mut dfs_stk = Vec::with_capacity(NUM_POLYGONAL_KINDS);

    loop {
        // Fix the octagonal numbers as the first kind in the cycle because:
        // - there are the fewest of them, reducing the branching factor at the root.
        // - fixing one kind does not exclude any solution as the cycle is rotationally symmetric.
        dfs_stk.push(DfsFrame {
            kind_offset: NUM_POLYGONAL_KINDS - 1,
            cursor: 0,
            end: polygonal_numbers_list[NUM_POLYGONAL_KINDS - 1].len(),
        });

        while let Some(frame) = dfs_stk.pop() {
            let cursor = frame.cursor;
            let kind_offset = frame.kind_offset;

            if cursor >= frame.end {
                continue;
            }

            dfs_stk.push(DfsFrame {
                cursor: cursor + 1,
                ..frame
            });

            if dfs_stk.len() == NUM_POLYGONAL_KINDS {
                let last_frame = &dfs_stk[NUM_POLYGONAL_KINDS - 1];
                let first_frame = &dfs_stk[0];

                let last_suffix = suffix(
                    polygonal_numbers_list[last_frame.kind_offset][last_frame.cursor - 1]
                );

                let first_prefix = prefix(
                    polygonal_numbers_list[first_frame.kind_offset][first_frame.cursor - 1]
                );

                if last_suffix == first_prefix {
                    return dfs_stk
                        .iter()
                        .map(|frame| polygonal_numbers_list[frame.kind_offset][frame.cursor - 1])
                        .sum::<u64>() as i64;
                }

                continue;
            }

            let next_kind_offset = kind_offsets[dfs_stk.len() - 1];
            let next_polygonal_numbers = &polygonal_numbers_list[next_kind_offset];

            let next_prefix = suffix(polygonal_numbers_list[kind_offset][cursor]);
            let lower_bound = next_prefix * PREFIX_DIVISOR;
            let upper_bound = lower_bound + PREFIX_DIVISOR;

            let next_cursor = next_polygonal_numbers.partition_point(|&polygonal| polygonal < lower_bound);
            let next_end = next_polygonal_numbers.partition_point(|&polygonal| polygonal < upper_bound);

            dfs_stk.push(DfsFrame {
                kind_offset: next_kind_offset,
                cursor: next_cursor,
                end: next_end,
            });
        }

        if !permutation::next_permutation(&mut kind_offsets) {
            break;
        }
    }

    unreachable!("answer could not be found");
}

// Returns the `n`th `3 + kind_offset`-gonal number.
// For example, `polygonal_number(0, n)` is the `n`th triangular number,
// `polygonal_number(1, n)` is the `n`th square number, etc.
// Note that `polygonal_number(kind_offset, 0)` is `0`.
fn polygonal_number(kind_offset: usize, n: usize) -> u64 {
    // Avoid underflow in the formula when `n == 0`.
    if n == 0 {
        return 0;
    }

    let kind_offset = kind_offset as u64;
    let n = n as u64;

    n * ((kind_offset + 1) * n + 1 - kind_offset) / 2
}

// Returns the `3 + kind_offset`-gonal numbers in [`begin`, `end`) in ascending order.
fn polygonal_numbers(kind_offset: usize, begin: u64, end: u64) -> Vec<u64> {
    let mut n = 0;
    let mut x = polygonal_number(kind_offset, n);

    while x < begin {
        n += 1;
        x = polygonal_number(kind_offset, n);
    }

    let mut res = Vec::new();

    while x < end {
        res.push(x);

        n += 1;
        x = polygonal_number(kind_offset, n);
    }

    res
}

fn prefix(x: u64) -> u64 {
    x / PREFIX_DIVISOR
}

fn suffix(x: u64) -> u64 {
    x % SUFFIX_MODULUS
}
