use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, test_cases) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Input must have two parts separated by an empty line");

    let pattern_set: HashSet<&str> = patterns.split(", ").collect();
    let mut memo: HashMap<String, bool> = HashMap::new();

    fn dfs(s: &str, patterns: &HashSet<&str>, memo: &mut HashMap<String, bool>) -> bool {
        if let Some(&cached) = memo.get(s) {
            return cached;
        }
        let result = s.is_empty()
            || (0..s.len())
                .any(|i| patterns.contains(&s[..=i]) && dfs(&s[i + 1..], patterns, memo));
        memo.insert(s.to_string(), result);
        result
    }

    let count = test_cases
        .lines()
        .filter(|&line| dfs(line, &pattern_set, &mut memo))
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<i128> {
    let (patterns, test_cases) = input
        .split("\n\n")
        .collect_tuple()
        .expect("should have an empty line");

    let pattern_set: HashSet<&str> = patterns.split(", ").collect();
    let mut memoization = HashMap::new();

    let total_sum = test_cases
        .lines()
        .map(|line| {
            fn dfs<'a>(
                s: &'a str,
                patterns: &HashSet<&'a str>,
                memo: &mut HashMap<&'a str, i128>,
            ) -> i128 {
                if let Some(&cached_result) = memo.get(s) {
                    return cached_result;
                }

                if s.is_empty() {
                    return 1;
                }

                let result = (0..s.len())
                    .filter_map(|i| {
                        if patterns.contains(&s[..=i]) {
                            Some(dfs(&s[i + 1..], patterns, memo))
                        } else {
                            None
                        }
                    })
                    .sum();

                memo.insert(s, result);
                result
            }

            dfs(line, &pattern_set, &mut memoization)
        })
        .sum();

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
