use glam::IVec2;
use pathfinding::prelude::bfs;
use std::collections::HashSet;

advent_of_code::solution!(18);

fn in_bounds(pos: &IVec2, min_bounds: &IVec2, max_bounds: &IVec2) -> bool {
    pos.x >= min_bounds.x && pos.x <= max_bounds.x && pos.y >= min_bounds.y && pos.y <= max_bounds.y
}

#[allow(dead_code)]
fn draw_map(corrupted: &HashSet<IVec2>, start: &IVec2, goal: &IVec2, path: &[IVec2]) {
    let mut map = String::new();
    for y in start.y..=goal.y {
        for x in start.x..=goal.x {
            let pos = IVec2::new(x, y);
            if corrupted.contains(&pos) {
                map.push('#');
            } else if path.contains(&pos) {
                map.push('O');
            } else {
                map.push('.');
            }
        }
        map.push('\n');
    }
    println!("{}", map);
}

pub fn part_one(input: &str) -> Option<u32> {
    let simulated_bytes = if cfg!(test) { 12 } else { 1024 };

    let corrupted = input
        .lines()
        .take(simulated_bytes)
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| IVec2::new(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let start = IVec2::new(0, 0);
    let goal = if cfg!(test) {
        IVec2::new(6, 6)
    } else {
        IVec2::new(70, 70)
    };

    let result = bfs(
        &start,
        |pos| {
            let mut neighbors = Vec::new();
            for dir in &[
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ] {
                let new_pos = pos + dir;
                if in_bounds(&new_pos, &start, &goal) && !corrupted.contains(&new_pos) {
                    neighbors.push(new_pos);
                }
            }
            neighbors
        },
        |pos| *pos == goal,
    )
    .unwrap();

    Some(result.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<String> {
    let corrupted = input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| IVec2::new(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let start = IVec2::new(0, 0);
    let goal = if cfg!(test) {
        IVec2::new(6, 6)
    } else {
        IVec2::new(70, 70)
    };

    let start_bytes = if cfg!(test) { 12 } else { 1024 };

    let first_unreachable = (start_bytes..corrupted.len())
        .find(|&simulated_bytes| {
            let corrupted = corrupted
                .iter()
                .take(simulated_bytes)
                .copied()
                .collect::<HashSet<_>>();

            let result = bfs(
                &start,
                |pos| {
                    let mut neighbors = Vec::new();
                    for dir in &[
                        IVec2::new(0, 1),
                        IVec2::new(0, -1),
                        IVec2::new(1, 0),
                        IVec2::new(-1, 0),
                    ] {
                        let new_pos = pos + dir;
                        if in_bounds(&new_pos, &start, &goal) && !corrupted.contains(&new_pos) {
                            neighbors.push(new_pos);
                        }
                    }
                    neighbors
                },
                |pos| *pos == goal,
            );

            result.is_none()
        })
        .unwrap();

    Some(format!(
        "{},{}",
        corrupted[first_unreachable - 1].x,
        corrupted[first_unreachable - 1].y
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
