advent_of_code::solution!(2);

pub fn is_safe(input: &[i32]) -> bool {
    let all_positive_or_negative = input.iter().all(|&x| x > 0) || input.iter().all(|&x| x < 0);
    let within_bounds = input.iter().all(|&x| x.abs() > 0 && x.abs() < 4);

    all_positive_or_negative && within_bounds
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            let diffs: Vec<i32> = nums.windows(2).map(|x| x[0] - x[1]).collect();

            is_safe(&diffs)
        })
        .count();

    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            let diffs: Vec<i32> = nums.windows(2).map(|x| x[0] - x[1]).collect();

            if is_safe(&diffs) {
                true
            } else {
                for i in 0..nums.len() {
                    let mut temp_nums = nums.clone();
                    temp_nums.remove(i);
                    let tmp_diffs: Vec<i32> = temp_nums.windows(2).map(|x| x[0] - x[1]).collect();

                    if is_safe(&tmp_diffs) {
                        return true;
                    }
                }

                false
            }
        })
        .count();

    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
