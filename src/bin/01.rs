advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let mut vals = line
                .split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok());
            (vals.next().unwrap(), vals.next().unwrap())
        })
        .collect();

    let mut sorted_list_1: Vec<u32> = pairs.iter().map(|&(x, _)| x).collect();
    let mut sorted_list_2: Vec<u32> = pairs.iter().map(|&(_, y)| y).collect();

    sorted_list_1.sort_unstable();
    sorted_list_2.sort_unstable();

    let total_distance = sorted_list_1
        .iter()
        .zip(sorted_list_2)
        .map(|(x, y)| x.abs_diff(y))
        .sum();

    // Output the results
    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let mut vals = line
                .split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok());
            (vals.next().unwrap(), vals.next().unwrap())
        })
        .collect();

    let similarity_score = pairs.iter().fold(0, |acc, &(i, _)| {
        acc + pairs.iter().filter(|&&(_, j)| i == j).count() as u32 * i
    });

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
