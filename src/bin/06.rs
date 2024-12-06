use glam::IVec2;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Debug, Clone)]
enum PositionType {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    Up,
    Right,
    Down,
    Left,
}

const UP: IVec2 = IVec2::new(0, -1);
const RIGHT: IVec2 = IVec2::new(1, 0);
const DOWN: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);

impl Facing {
    fn to_vec2(self) -> IVec2 {
        match self {
            Facing::Up => UP,
            Facing::Right => RIGHT,
            Facing::Down => DOWN,
            Facing::Left => LEFT,
        }
    }

    fn to_facing(self, dir: char) -> Facing {
        match dir {
            'L' => match self {
                Facing::Up => Facing::Left,
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
            },
            'R' => match self {
                Facing::Up => Facing::Right,
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
            },
            _ => panic!("Invalid direction"),
        }
    }
}

fn run_guard_simulation(
    min_bounds: &IVec2,
    max_bounds: &IVec2,
    grid: &HashMap<IVec2, PositionType>,
    guard_starting_position: IVec2,
    guard_starting_facing: Facing,
) -> Option<Vec<(IVec2, Facing)>> {
    let mut visited: Vec<(IVec2, Facing)> = Vec::new();
    let mut guard_pos = guard_starting_position;
    let mut guard_facing = guard_starting_facing;

    visited.push((guard_pos, guard_facing));

    while guard_pos.x >= min_bounds.x
        && guard_pos.y >= min_bounds.y
        && guard_pos.x <= max_bounds.x
        && guard_pos.y <= max_bounds.y
    {
        let next_pos = guard_pos + guard_facing.to_vec2();
        let next_pos_type = grid.get(&next_pos);

        match next_pos_type {
            Some(PositionType::Empty) => {
                if visited.contains(&(next_pos, guard_facing)) {
                    return None;
                }

                guard_pos = next_pos;
                visited.push((guard_pos, guard_facing));
            }
            Some(PositionType::Obstacle) => {
                guard_facing = guard_facing.to_facing('R');
            }
            None => {
                break;
            }
        }
    }

    Some(visited)
}

fn parse_grid(input: &str) -> (HashMap<IVec2, PositionType>, IVec2, Facing, IVec2, IVec2) {
    // parse the input as a 2D grid stored in a HashMap of Vec2 and PositionType
    let mut grid = HashMap::new();
    let mut guard_pos = IVec2::new(0, 0);
    let guard_facing = Facing::Up;

    // get bounds of the grid
    let min_bounds = IVec2::new(0, 0);
    let max_bounds = IVec2::new(
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            let pos_type = match c {
                '.' => PositionType::Empty,
                '^' => PositionType::Empty,
                '#' => PositionType::Obstacle,
                _ => panic!("Invalid character in input"),
            };

            if c == '^' {
                guard_pos = pos;
            }

            grid.insert(pos, pos_type);
        }
    }

    (grid, guard_pos, guard_facing, min_bounds, max_bounds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, guard_pos, guard_facing, min_bounds, max_bounds) = parse_grid(input);

    let visited = run_guard_simulation(&min_bounds, &max_bounds, &grid, guard_pos, guard_facing)?;
    let unique_positions: HashSet<IVec2> = visited.iter().map(|(pos, _)| pos).cloned().collect();

    Some(unique_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, guard_pos, guard_facing, min_bounds, max_bounds) = parse_grid(input);

    let visited = run_guard_simulation(&min_bounds, &max_bounds, &grid, guard_pos, guard_facing)?;
    let unique_positions: HashSet<IVec2> = visited
        .iter()
        .filter_map(|(pos, _)| {
            if *pos == guard_pos {
                return None;
            }
            Some(*pos)
        })
        .collect();

    let found = unique_positions
        .par_iter()
        .filter_map(|pos| {
            let mut new_grid = grid.clone();
            new_grid.insert(*pos, PositionType::Obstacle);

            let result =
                run_guard_simulation(&min_bounds, &max_bounds, &new_grid, guard_pos, guard_facing);
            if result.is_none() {
                Some(*pos)
            } else {
                None
            }
        })
        .count();

    Some(found as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}