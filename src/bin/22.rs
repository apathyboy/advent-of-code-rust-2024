use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(22);

fn mix(secret: u64, mixin: u64) -> u64 {
    mixin ^ secret
}

fn prune(secret: u64) -> u64 {
    secret.rem_euclid(16777216)
}

fn evolve_secret(mut secret: u64) -> u64 {
    let step1 = secret * 64;
    secret = prune(mix(secret, step1));

    let step2 = (secret as f64 / 32.0).floor() as u64;
    secret = prune(mix(secret, step2));

    let step3 = secret * 2048;
    secret = prune(mix(secret, step3));

    secret
}

pub fn part_one(input: &str) -> Option<u64> {
    let secret_sum = input
        .par_lines()
        .map(|line| {
            let mut secret = line.parse::<u64>().unwrap();

            for _ in 0..2000 {
                secret = evolve_secret(secret);
            }

            secret
        })
        .sum();
    Some(secret_sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    /*
    let change_sequences: Vec<Vec<(i32, u64)>> = input
        .lines()
        .map(|line| {
            let starting_secret = line.parse::<u64>().unwrap();

            let mut secret_numbers = vec![starting_secret
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as u64];

            let mut secret = starting_secret;

            secret_numbers.extend((0..2000).map(|_| {
                secret = evolve_secret(secret);
                secret
                    .to_string()
                    .chars()
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as u64
            }));

            let deltas: Vec<i32> = secret_numbers
                .windows(2)
                .map(|w| w[1] as i32 - w[0] as i32)
                .collect();

            deltas
                .into_iter()
                .zip(secret_numbers.into_iter().skip(1))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut max_bananas = 0;

    for i in 0..change_sequences.len() {
        println!("Checking sequence {}", i);

        let seq1 = change_sequences[i].clone();

        for w in seq1.windows(4) {
            let mut bananas = w[3].1;

            for j in 0..change_sequences.len() {
                if i == j {
                    continue;
                }

                for w2 in change_sequences[j].windows(4) {
                    if w[0].0 == w2[0].0
                        && w[1].0 == w2[1].0
                        && w[2].0 == w2[2].0
                        && w[3].0 == w2[3].0
                    {
                        bananas += w2[3].1;
                        break;
                    }
                }
            }

            if bananas > max_bananas {
                max_bananas = bananas;
            }
        }

        println!("Max bananas: {}", max_bananas);
    }
    */

    let mut p2 = HashMap::new();
    let mut seen = HashSet::new();
    for l in input.lines() {
        let mut ps = [0; 2001];
        let mut p = l.parse().unwrap();
        ps[0] = p;
        for i in 0..2000 {
            p = (p ^ (p * 64)) % 16777216;
            p = (p ^ (p / 32)) % 16777216;
            p = (p ^ (p * 2048)) % 16777216;
            ps[i + 1] = p;
        }

        for p in &mut ps {
            *p %= 10;
        }
        seen.clear();
        for (a, b, c, d, e) in ps.iter().tuple_windows() {
            let k = (b - a) + (c - b) * 100 + (d - c) * 10000 + (e - d) * 1000000;
            if seen.insert(k) {
                *p2.entry(k).or_default() += *e;
            }
        }
    }

    Some(*p2.values().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
