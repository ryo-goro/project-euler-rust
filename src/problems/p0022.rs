// Comma-separated list of quoted names.
const NAMES: &str = include_str!("../../data/p0022_names.txt");

pub fn solve() -> u64 {
    let mut names: Vec<_> = NAMES
        .split(',')
        .map(|quoted_name| {
            quoted_name
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
        })
        .collect();

    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| ((i + 1) as u64) * name_score(name))
        .sum()
}

fn name_score(name: &str) -> u64 {
    name
        .as_bytes()
        .iter()
        .map(|byte| (byte - b'A' + 1) as u64)
        .sum()
}
