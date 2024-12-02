advent_of_code::solution!(2);

fn is_safe(input: &[i32]) -> bool {
    let diffs: Vec<i32> = input.windows(2).map(|x| x[0] - x[1]).collect();
    let all_positive_or_negative = diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0);
    let within_bounds = diffs.iter().all(|&x| x.abs() > 0 && x.abs() < 4);

    all_positive_or_negative && within_bounds
}

fn is_safe_with_tolerance(input: &[i32]) -> bool {
    if is_safe(&input) {
        return true;
    }

    for i in 0..input.len() {
        let mut temp_nums = input.to_vec();
        temp_nums.remove(i);

        if is_safe(&temp_nums) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            is_safe(&nums)
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

            is_safe_with_tolerance(&nums)
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
