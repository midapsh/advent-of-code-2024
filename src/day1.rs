use std::collections::HashMap;

const _DUMMY_INPUT: &str = include_str!("data/day1-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day1-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let (list1, list2) = get_lists(values);
    let mut dist = 0;
    for (v1, v2) in list1.into_iter().zip(list2.into_iter()) {
        dist += (v1 - v2).abs()
    }

    dist.to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let (list1, list2) = get_lists(values);

    let mut mapper = HashMap::with_capacity(1000);

    for v in list2 {
        mapper.entry(v).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut result = 0;
    for v in list1 {
        if let Some(&count) = mapper.get(&v) {
            result += v * count;
        }
    }

    result.to_string()
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

fn get_lists(values: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::with_capacity(1000);
    let mut list2 = Vec::with_capacity(1000);
    for line in values.lines() {
        let mut values = line.split_ascii_whitespace();
        list1.push(values.next().unwrap().parse::<i32>().unwrap());
        list2.push(values.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort_unstable();
    list2.sort_unstable();

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("11", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("31", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        // 1765812
        println!("{}", solve_part_1_real());
    }
    #[test]
    fn test_part_2_real() {
        // 20520794
        println!("{}", solve_part_2_real());
    }
}
