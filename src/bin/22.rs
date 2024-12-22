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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
