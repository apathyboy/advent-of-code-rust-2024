use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = re
        .captures_iter(input)
        .map(|cap| {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();
            a * b
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"don't\(\)[\S\s]*?do\(\)").unwrap();
    let new_input = re.replace_all(input, "");

    new_input
        .find("don't()")
        .map(|pos| part_one(&new_input[..pos]))
        .unwrap_or_else(|| part_one(&new_input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
