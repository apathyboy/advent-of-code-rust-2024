use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(10);

fn neighbors<'a>(
    pos: IVec2,
    current_height: u32,
    graph: &'a HashMap<IVec2, u32>,
) -> impl Iterator<Item = (IVec2, u32)> + 'a {
    const DIRECTIONS: [IVec2; 4] = [
        IVec2::new(0, 1),
        IVec2::new(0, -1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
    ];

    DIRECTIONS
        .iter()
        .map(move |&dir| pos + dir)
        .filter_map(move |new_pos| graph.get(&new_pos).map(|&height| (new_pos, height)))
        .filter(move |&(_, height)| height == current_height + 1)
}

fn explore(
    pos: IVec2,
    current_height: u32,
    graph: &HashMap<IVec2, u32>,
    visited: &mut HashSet<IVec2>,
    found: &mut Vec<IVec2>,
) {
    if current_height == 9 {
        found.push(pos);
        return;
    }

    for (neighbor, height) in neighbors(pos, current_height, graph) {
        if visited.insert(neighbor) {
            explore(neighbor, height, graph, visited, found);
            visited.remove(&neighbor); // Backtracking
        }
    }
}

fn parse_map(input: &str) -> HashMap<IVec2, u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let pos = IVec2::new(x as i32, y as i32);
                let height = c.to_digit(10).unwrap();
                (pos, height)
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: HashMap<IVec2, u32> = parse_map(input);
    let trailheads: Vec<IVec2> = map
        .iter()
        .filter_map(|(pos, height)| if *height == 0 { Some(*pos) } else { None })
        .collect();

    let score = trailheads
        .iter()
        .map(|p| {
            let mut visited = HashSet::new();
            let mut found = Vec::new();

            explore(*p, 0, &map, &mut visited, &mut found);

            found.iter().unique().count() as u32
        })
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: HashMap<IVec2, u32> = parse_map(input);
    let trailheads: Vec<IVec2> = map
        .iter()
        .filter_map(|(pos, height)| if *height == 0 { Some(*pos) } else { None })
        .collect();

    let score = trailheads
        .iter()
        .map(|p| {
            let mut visited = HashSet::new();
            let mut found = Vec::new();

            explore(*p, 0, &map, &mut visited, &mut found);

            found.len() as u32
        })
        .sum();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
