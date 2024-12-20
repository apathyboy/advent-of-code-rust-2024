use glam::IVec2;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

fn parse(input: &str) -> (HashSet<IVec2>, IVec2, IVec2) {
    let mut grid = HashSet::new();
    let mut start = IVec2::new(0, 0);
    let mut end = IVec2::new(0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(IVec2::new(j as i32, i as i32));
            } else if c == 'S' {
                start = IVec2::new(j as i32, i as i32);
            } else if c == 'E' {
                end = IVec2::new(j as i32, i as i32);
            }
        }
    }

    (grid, start, end)
}

fn bfs(maze: &HashSet<IVec2>, start: &IVec2, end: &IVec2) -> HashMap<IVec2, usize> {
    let mut queue = VecDeque::from([(*start, 0)]);
    let mut dists = HashMap::new();

    while !queue.is_empty() {
        let (pos, dist) = queue.pop_front().unwrap();

        if dists.contains_key(&pos) {
            continue;
        }

        dists.insert(pos, dist);

        if pos == *end {
            continue;
        }

        for dir in &[IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
            let new_pos = pos + dir;
            if !maze.contains(&new_pos) {
                queue.push_back((new_pos, dist + 1));
            }
        }
    }

    dists
}

pub fn part_one(input: &str) -> Option<u32> {
    let (maze, start, end) = parse(input);
    let dists: Vec<_> = bfs(&maze, &start, &end).into_iter().collect();

    let saved_seconds = if cfg!(test) { 64 } else { 100 };

    let p1 = (0..dists.len())
        .into_par_iter() // Parallelize the outer loop
        .map(|i| {
            let (pos1, dist1) = dists[i];
            let mut local_count = 0;

            for (pos2, dist2) in dists.iter().skip(i + 1) {
                let d = pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y);

                if d <= 2 && dist2.abs_diff(dist1) >= d as usize + saved_seconds {
                    local_count += 1;
                }
            }

            local_count
        })
        .sum::<usize>();

    Some(p1 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (maze, start, end) = parse(input);
    let dists: Vec<_> = bfs(&maze, &start, &end).into_iter().collect();

    let saved_seconds = if cfg!(test) { 76 } else { 100 };

    let p1 = (0..dists.len())
        .into_par_iter() // Parallelize the outer loop
        .map(|i| {
            let (pos1, dist1) = dists[i];
            let mut local_count = 0;

            for (pos2, dist2) in dists.iter().skip(i + 1) {
                let d = pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y);

                if d <= 20 && dist2.abs_diff(dist1) >= d as usize + saved_seconds {
                    local_count += 1;
                }
            }

            local_count
        })
        .sum::<usize>();

    Some(p1 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
