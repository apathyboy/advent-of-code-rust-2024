use glam::IVec2;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn in_bounds(pos: IVec2, min_bounds: IVec2, max_bounds: IVec2) -> bool {
    pos.x >= min_bounds.x && pos.x < max_bounds.x && pos.y >= min_bounds.y && pos.y < max_bounds.y
}

fn parse_grid(input: &str) -> (HashMap<char, Vec<IVec2>>, IVec2, IVec2) {
    let mut grid = HashMap::new();
    let min_bounds = IVec2::new(0, 0);
    let max_bounds = IVec2::new(
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            grid.entry(c)
                .or_insert_with(Vec::new)
                .push(IVec2::new(x as i32, y as i32));
        }
    }
    (grid, min_bounds, max_bounds)
}

fn find_antinodes(
    grid: &HashMap<char, Vec<IVec2>>,
    min_bounds: IVec2,
    max_bounds: IVec2,
    infinite: bool,
) -> HashSet<IVec2> {
    let mut antinodes = HashSet::new();

    for (_, positions) in grid.iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                // offset 1
                let offset1 = positions[i] - positions[j];
                let mut last_pos = positions[i];

                while in_bounds(last_pos + offset1, min_bounds, max_bounds) {
                    last_pos += offset1;
                    antinodes.insert(last_pos);

                    if !infinite {
                        break;
                    }
                }

                // offset 2
                let offset2 = positions[j] - positions[i];
                let mut last_pos = positions[j];

                while in_bounds(last_pos + offset2, min_bounds, max_bounds) {
                    last_pos += offset2;
                    antinodes.insert(last_pos);

                    if !infinite {
                        break;
                    }
                }
            }
        }
    }

    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, min_bounds, max_bounds) = parse_grid(input);

    let antinodes = find_antinodes(&grid, min_bounds, max_bounds, false);

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, min_bounds, max_bounds) = parse_grid(input);

    let antinodes = find_antinodes(&grid, min_bounds, max_bounds, true);

    let antennas = grid
        .iter()
        .flat_map(|(_, positions)| positions.clone())
        .collect::<Vec<IVec2>>();

    // Combine the vectors
    let combined: HashSet<_> = antinodes.into_iter().chain(antennas).collect();

    Some(combined.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
