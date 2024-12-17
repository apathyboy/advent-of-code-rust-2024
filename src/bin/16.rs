use glam::IVec2;
use pathfinding::prelude::{astar_bag, dijkstra};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(16);

const EAST: IVec2 = IVec2::new(1, 0);
const WEST: IVec2 = IVec2::new(-1, 0);
const NORTH: IVec2 = IVec2::new(0, -1);
const SOUTH: IVec2 = IVec2::new(0, 1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Reindeer {
    pos: IVec2,
    dir: IVec2,
}

impl Reindeer {
    fn new(pos: IVec2, dir: IVec2) -> Self {
        Self { pos, dir }
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            EAST => NORTH,
            NORTH => WEST,
            WEST => SOUTH,
            SOUTH => EAST,
            _ => panic!("Invalid direction"),
        };
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            EAST => SOUTH,
            SOUTH => WEST,
            WEST => NORTH,
            NORTH => EAST,
            _ => panic!("Invalid direction"),
        };
    }

    fn move_forward(&mut self) {
        self.pos += self.dir;
    }
}

fn move_forward(reindeer: &mut Reindeer) -> u32 {
    reindeer.move_forward();
    1
}

fn turn_left_and_move(reindeer: &mut Reindeer) -> u32 {
    reindeer.turn_left();
    reindeer.move_forward();
    1001
}

fn turn_right_and_move(reindeer: &mut Reindeer) -> u32 {
    reindeer.turn_right();
    reindeer.move_forward();
    1001
}

fn successors(reindeer: &Reindeer, maze: &HashMap<IVec2, char>) -> Vec<(Reindeer, u32)> {
    // Define a list of transformations and their associated costs
    let moves: [fn(&mut Reindeer) -> u32; 3] = [
        move_forward,        // Move forward
        turn_left_and_move,  // Turn left and move
        turn_right_and_move, // Turn right and move
    ];

    moves
        .iter()
        .filter_map(|&action| {
            let mut next_reindeer = *reindeer; // Clone the current reindeer
            let cost = action(&mut next_reindeer); // Apply the transformation and get the cost

            // Check if the move is valid
            (!maze.contains_key(&next_reindeer.pos)).then_some((next_reindeer, cost))
        })
        .collect()
}

#[allow(unused)]
fn draw_maze(grid: &HashMap<IVec2, char>, start: IVec2, end: IVec2) {
    let mut min = IVec2::new(0, 0);
    let mut max = IVec2::new(0, 0);

    for pos in grid.keys() {
        min = min.min(*pos);
        max = max.max(*pos);
    }

    for j in min.y..=max.y {
        for i in min.x..=max.x {
            let pos = IVec2::new(i, j);
            if pos == start {
                print!("S");
            } else if pos == end {
                print!("E");
            } else if let Some(c) = grid.get(&pos) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(unused)]
fn draw_maze_walk(grid: &HashMap<IVec2, char>, start: IVec2, end: IVec2, path: Vec<Reindeer>) {
    let mut min = IVec2::new(0, 0);
    let mut max = IVec2::new(0, 0);

    for pos in grid.keys() {
        min = min.min(*pos);
        max = max.max(*pos);
    }

    for j in min.y..=max.y {
        for i in min.x..=max.x {
            let pos = IVec2::new(i, j);
            if pos == start {
                print!("S");
            } else if pos == end {
                print!("E");
            } else if let Some(c) = grid.get(&pos) {
                print!("{}", c);
            } else if path.iter().any(|r| r.pos == pos) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse(input: &str) -> (HashMap<IVec2, char>, IVec2, IVec2) {
    let mut grid = HashMap::new();
    let mut start = IVec2::new(0, 0);
    let mut end = IVec2::new(0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(IVec2::new(j as i32, i as i32), c);
            } else if c == 'S' {
                start = IVec2::new(j as i32, i as i32);
            } else if c == 'E' {
                end = IVec2::new(j as i32, i as i32);
            }
        }
    }

    (grid, start, end)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (maze, start, end) = parse(input);

    let reindeer = Reindeer::new(start, EAST);

    dijkstra(&reindeer, |r| successors(r, &maze), |&r| r.pos == end).map(|result| result.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (maze, start, end) = parse(input);

    let reindeer = Reindeer::new(start, EAST);

    let result = astar_bag(
        &reindeer,
        |r| successors(r, &maze),
        |_| 0,
        |&r| r.pos == end,
    );

    let mut seats = HashSet::new();
    match result {
        Some((path, _)) => {
            seats.extend(path.into_iter().flatten().map(|v| v.pos));
        }
        None => {
            println!("No path found.");
        }
    }

    Some(seats.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
