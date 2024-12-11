use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Stones {
    counts: HashMap<usize, usize>,
    size: usize,
}

impl Stones {
    fn new(stones: &[usize]) -> Self {
        let counts = stones.iter().fold(HashMap::new(), |mut counts, &n| {
            *counts.entry(n).or_insert(0) += 1;
            counts
        });

        Self {
            counts,
            size: stones.len(),
        }
    }

    fn blink(&mut self) {
        let mut new_counts = HashMap::new();
        for (&n, &count) in &self.counts {
            match halve(n) {
                Some((a, b)) => {
                    *new_counts.entry(a).or_insert(0) += count;
                    *new_counts.entry(b).or_insert(0) += count;
                    self.size += count;
                }
                None if n == 0 => {
                    *new_counts.entry(1).or_insert(0) += count;
                }
                None => {
                    *new_counts.entry(n * 2024).or_insert(0) += count;
                }
            }
        }
        self.counts = new_counts;
    }
}

fn halve(n: usize) -> Option<(usize, usize)> {
    if n == 0 {
        return None;
    }

    let digits = n.ilog10() + 1;
    if digits % 2 == 0 {
        let div = 10usize.pow(digits / 2);
        Some((n / div, n % div))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut stones = Stones::new(&stones);

    for _ in 0..25 {
        stones.blink();
    }

    Some(stones.size as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut stones = Stones::new(&stones);

    for _ in 0..75 {
        stones.blink();
    }

    Some(stones.size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
