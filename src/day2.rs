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

fn is_safe(level: &[i32]) -> bool {
    let len = level.len();
    if len < 2 {
        return false;
    }

    let mut is_increasing = None;

    for window in level.windows(2) {
        let diff = window[1] - window[0];

        if !((1 <= diff.abs()) && (diff.abs() <= 3)) {
            return false;
        }

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
                    return false;
                }
            }
            Some(false) => {
                if diff >= 0 {
                    return false;
                }
            }
        }
    }

    true
}

fn can_be_made_safe(level: &[i32]) -> bool {
    let len = level.len();
    if len < 2 {
        return false;
    }

    for i in 0..len {
        let mut modified_level = level.to_vec();
        modified_level.remove(i);

        if is_safe(&modified_level) {
            return true;
        }
    }

    false
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
