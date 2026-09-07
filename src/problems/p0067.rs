const HEIGHT: usize = 100;
const TRIANGLE: &str = include_str!("../../data/p0067_triangle.txt");

pub fn solve() -> i64 {
    // Dynamic programming:
    // Update the maximum path sums in place, one row at a time.
    let mut max_path_sums = [0; HEIGHT];

    for row_values in TRIANGLE.split('\n') {
        let mut upper_left = 0;

        // max_path_sums[col] stores the maximum path sum to column `col`
        // in the current row.
        for (col, value) in row_values
            .split_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .enumerate()
        {
            let upper = max_path_sums[col];
            max_path_sums[col] = value + upper_left.max(upper);
            upper_left = upper;
        }
    }

    max_path_sums.into_iter().max().unwrap() as i64
}
