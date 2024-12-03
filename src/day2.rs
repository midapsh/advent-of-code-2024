const _DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day2-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let levels = parse_values(values);
    let mut safe_count = 0;

    for level in levels {
        if is_safe(&level) {
            safe_count += 1;
        }
    }

    safe_count.to_string()
}
fn private_solve_part_2(values: &str) -> String {
    let levels = parse_values(values);
    let mut safe_count = 0;

    for level in levels {
        if is_safe(&level) || can_be_made_safe(&level) {
            safe_count += 1;
        }
    }

    safe_count.to_string()
}

fn can_be_made_safe(level: &[i32]) -> bool {
    if level.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = level.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let mut bad_indices = Vec::new();

    for (i, &diff) in diffs.iter().enumerate() {
        if !is_valid_difference(diff) {
            bad_indices.push(i);
        }
    }

    // NOTE(hspadim): If there is more than one bad difference, it cannot be fixed
    if bad_indices.len() > 1 {
        return false;
    }

    // NOTE(hspadim): Should not happen since the input is already unsafe
    if bad_indices.is_empty() {
        return false;
    }

    let bad_index = bad_indices[0];

    // NOTE(hspadim): Removing the problematic level and checking if the sequence becomes safe
    let mut modified_level = level.to_vec();
    if bad_index == 0 {
        // NOTE(hspadim): Remove the first level (invalid transition with second level)
        modified_level.remove(0);
    } else {
        // NOTE(hspadim): Remove the level after the bad difference
        modified_level.remove(bad_index + 1);
    }

    is_safe(&modified_level)
}

fn is_valid_difference(diff: i32) -> bool {
    (1..=3).contains(&diff) || (-3..=-1).contains(&diff)
}

fn is_safe(level: &[i32]) -> bool {
    if level.len() < 2 {
        return false; // A safe report must have at least two levels.
    }

    let mut is_increasing = None; // None: undetermined, Some(true): increasing, Some(false): decreasing

    for window in level.windows(2) {
        let diff = window[1] - window[0];

        // Check difference constraints
        if !is_valid_difference(diff) {
            return false;
        }

        // Determine increasing/decreasing direction
        match is_increasing {
            None => {
                if diff > 0 {
                    is_increasing = Some(true);
                } else if diff < 0 {
                    is_increasing = Some(false);
                }
            }
            Some(true) => {
                if diff <= 0 {
                    return false; // Expected strictly increasing
                }
            }
            Some(false) => {
                if diff >= 0 {
                    return false; // Expected strictly decreasing
                }
            }
        }
    }

    true
}

fn _solve_part_1_dummy() -> String {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY_INPUT)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

fn parse_values(values: &str) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for v in values.lines() {
        let mut temp = vec![];
        for number in v.split_ascii_whitespace() {
            temp.push(number.parse::<i32>().unwrap());
        }
        ans.push(temp)
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("2", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("4", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 472
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real()); // 520
    }
}
