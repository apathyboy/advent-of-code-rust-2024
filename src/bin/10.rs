use glam::IVec2;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(10);

fn neighbors(pos: IVec2, current_height: u32, graph: &HashMap<IVec2, u32>) -> Vec<(IVec2, u32)> {
    let mut neighbors = Vec::new();

    for dir in &[
        IVec2::new(0, 1),
        IVec2::new(0, -1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
    ] {
        let new_pos = pos + *dir;
        if let Some(height) = graph.get(&new_pos) {
            if height == &(current_height + 1) {
                neighbors.push((new_pos, *height));
            }
        }
    }

    neighbors
}

fn explore(
    pos: IVec2,
    current_height: u32,
    graph: &HashMap<IVec2, u32>,
    visited: &mut HashSet<IVec2>,
    found: &mut HashSet<IVec2>,
) {
    visited.insert(pos);

    for (neighbor, height) in neighbors(pos, current_height, graph) {
        if visited.contains(&neighbor) {
            continue;
        }

        if height == 9 {
            found.insert(neighbor);
            continue;
        }

        explore(neighbor, height, graph, visited, found);
    }
}

fn explore2(
    pos: IVec2,
    current_height: u32,
    graph: &HashMap<IVec2, u32>,
    visited: &mut HashSet<IVec2>,
    count: &mut usize,
) {
    if current_height == 9 {
        *count += 1;
        return;
    }

    for (neighbor, height) in neighbors(pos, current_height, graph) {
        if visited.contains(&neighbor) {
            continue;
        }

        visited.insert(neighbor);
        explore2(neighbor, height, graph, visited, count);
        visited.remove(&neighbor);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse the input into a hashmap of Vec2 positions as index and the values parsed as u32 values
    let mut map: HashMap<IVec2, u32> = HashMap::new();
    let mut trailheads: Vec<IVec2> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            let pos = IVec2::new(x as i32, y as i32);

            map.insert(pos, height);

            if height == 0 {
                trailheads.push(pos);
            }
        }
    }

    let mut score = 0;

    for trailhead in trailheads.iter() {
        let mut visited = HashSet::new();
        let mut found = HashSet::new();

        explore(*trailhead, 0, &map, &mut visited, &mut found);

        score += found.len() as u32;
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse the input into a hashmap of Vec2 positions as index and the values parsed as u32 values
    let mut map: HashMap<IVec2, u32> = HashMap::new();
    let mut trailheads: Vec<IVec2> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            let pos = IVec2::new(x as i32, y as i32);

            map.insert(pos, height);

            if height == 0 {
                trailheads.push(pos);
            }
        }
    }

    let mut score = 0;

    for trailhead in trailheads.iter() {
        let mut visited = HashSet::new();
        let mut count = 0;

        explore2(*trailhead, 0, &map, &mut visited, &mut count);

        score += count;
    }

    Some(score as u32)
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
