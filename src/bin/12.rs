use glam::IVec2;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

const UP: IVec2 = IVec2::new(0, 1);
const DOWN: IVec2 = IVec2::new(0, -1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

fn parse_map(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let pos = IVec2::new(x as i32, y as i32);
                (pos, c)
            })
        })
        .collect()
}

fn find_regions(map: &HashMap<IVec2, char>) -> Vec<(char, Vec<IVec2>)> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new(); // Track visited positions to avoid redundant checks

    for (&pos, &c) in map {
        if visited.contains(&pos) {
            continue;
        }

        // Perform a breadth-first search to find all connected regions with the same character
        let mut region = Vec::new();
        let mut queue = vec![pos];

        while let Some(curr_pos) = queue.pop() {
            if !visited.insert(curr_pos) {
                continue; // Skip if already visited
            }

            region.push(curr_pos);

            for &dir in &[
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ] {
                let neighbor = curr_pos + dir;
                if let Some(&neighbor_c) = map.get(&neighbor) {
                    if neighbor_c == c {
                        queue.push(neighbor);
                    }
                }
            }
        }

        regions.push((c, region));
    }

    regions
}

fn price_region(region: &[IVec2]) -> u32 {
    let area = region.len() as u32;

    // loop through each area and count the sides that are not touching another region
    let mut perimeter = 0;
    for &pos in region {
        for &dir in &[
            IVec2::new(0, 1),
            IVec2::new(0, -1),
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
        ] {
            let new_pos = pos + dir;
            if !region.contains(&new_pos) {
                perimeter += 1;
            }
        }
    }

    area * perimeter
}

fn count_sides(region: &[IVec2]) -> usize {
    let mut sides: Vec<Vec<(IVec2, IVec2)>> = Vec::new();

    for i in 0..region.len() {
        for &dir in &[UP, DOWN, LEFT, RIGHT] {
            let pos = region[i];

            let check_open_position = pos + dir;

            if region.contains(&check_open_position) {
                continue;
            }

            if sides.iter().any(|side| side.contains(&(pos, dir))) {
                continue;
            }

            // if dir is up or down check for all contiguous sides left and right of the position to see if they are a valid continuation of the side
            let mut side = vec![(pos, dir)];

            if dir == UP || dir == DOWN {
                let mut left = pos + LEFT;
                let mut right = pos + RIGHT;

                while region.contains(&left) {
                    let check_open_position = left + dir;

                    if region.contains(&check_open_position) {
                        break;
                    }

                    side.push((left, dir));

                    left += LEFT;
                }

                while region.contains(&right) {
                    let check_open_position = right + dir;

                    if region.contains(&check_open_position) {
                        break;
                    }

                    side.push((right, dir));

                    right += RIGHT;
                }
            } else if dir == LEFT || dir == RIGHT {
                let mut up = pos + UP;
                let mut down = pos + DOWN;

                while region.contains(&up) {
                    let check_open_position = up + dir;

                    if region.contains(&check_open_position) {
                        break;
                    }

                    side.push((up, dir));

                    up += UP;
                }

                while region.contains(&down) {
                    let check_open_position = down + dir;

                    if region.contains(&check_open_position) {
                        break;
                    }

                    side.push((down, dir));

                    down += DOWN;
                }
            }

            sides.push(side);
        }
    }

    sides.len()
}

fn bulk_price_region(_name: &char, region: &[IVec2]) -> u32 {
    let area = region.len() as u32;
    let sides = count_sides(region);

    area * sides as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let pricing = find_regions(&map)
        .iter()
        .map(|(_, region)| price_region(region))
        .sum();

    Some(pricing)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let pricing = find_regions(&map)
        .iter()
        .map(|(name, region)| bulk_price_region(name, region))
        .sum();

    Some(pricing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }
}
