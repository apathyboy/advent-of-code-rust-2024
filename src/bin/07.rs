use rayon::prelude::*;

advent_of_code::solution!(7);

fn apply(op: char, a: u64, b: u64) -> u64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        '|' => (a * 10u64.pow(b.ilog10() + 1)) + b,
        _ => unreachable!(),
    }
}

fn try_evaluate(total: u64, nums: &mut [u64], ops: &[char]) -> bool {
    if nums.len() <= 1 {
        return nums[0] == total;
    }
    if nums[0] > total {
        return false;
    }

    for op in ops {
        let old = nums[1];
        nums[1] = apply(*op, nums[0], nums[1]);
        if try_evaluate(total, &mut nums[1..], ops) {
            return true;
        }
        nums[1] = old;
    }
    false
}

fn parse_equation(line: &str) -> (u64, Vec<u64>) {
    let (total, rest) = line.split_once(": ").unwrap();
    let nums = rest.split(' ').filter_map(|num| num.parse().ok()).collect();

    (total.parse().unwrap(), nums)
}

pub fn part_one(input: &str) -> Option<u64> {
    let opers = vec!['*', '+'];

    let calibration_result = input
        .lines()
        .par_bridge()
        .filter_map(|line| {
            let (total, mut nums) = parse_equation(line);

            if try_evaluate(total, &mut nums, &opers) {
                Some(total)
            } else {
                None
            }
        })
        .sum();

    Some(calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let opers = vec!['*', '+', '|'];

    let calibration_result = input
        .lines()
        .par_bridge()
        .filter_map(|line| {
            let (total, mut nums) = parse_equation(line);

            if try_evaluate(total, &mut nums, &opers) {
                Some(total)
            } else {
                None
            }
        })
        .sum();

    Some(calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
