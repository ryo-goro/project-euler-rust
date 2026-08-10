use std::collections::HashMap;

const TARGET_COUNT: usize = 5;

struct CubeGroup {
    smallest_cube: u64,
    count: usize,
}

pub fn solve() -> i64 {
    let mut cube_groups: HashMap<[u8; 10], CubeGroup> = HashMap::new();
    let mut digit_upper_bound = 10;

    let mut result: Option<u64> = None;

    for n in 1u64.. {
        let cube = n.pow(3);

        // All cubes with the previous number of digits have been processed.
        // If a group reached the target count, its smallest cube is the
        // smallest valid answer for that digit length.
        if cube >= digit_upper_bound {
            if let Some(smallest) = result {
                return smallest as i64;
            }

            while cube >= digit_upper_bound {
                digit_upper_bound *= 10;
            }

            cube_groups.clear();
            result = None;
        }

        let group = cube_groups.entry(digit_signature(cube))
            .and_modify(|group| group.count += 1)
            .or_insert(CubeGroup {
                smallest_cube: cube,
                count: 1,
            });
        
        if group.count == TARGET_COUNT {
            let candidate = group.smallest_cube;
            result = Some(result.map_or(candidate, |current| current.min(candidate)));
        }
    }

    unreachable!("answer could not be found");
}

fn digit_signature(mut n: u64) -> [u8; 10] {
    let mut counts = [0; 10];

    while n > 0 {
        counts[(n % 10) as usize] += 1;
        n /= 10;
    }

    counts
}
