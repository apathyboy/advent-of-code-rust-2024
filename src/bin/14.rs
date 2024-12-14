use glam::IVec2;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(14);

struct Trajectory {
    p: IVec2,
    v: IVec2,
}

impl Trajectory {
    fn new(p: IVec2, v: IVec2) -> Self {
        Self { p, v }
    }

    fn step(&mut self, extent: &IVec2) {
        self.p += self.v;

        self.p.x = self.p.x.rem_euclid(extent.x);
        self.p.y = self.p.y.rem_euclid(extent.y);
    }
}

fn parse_trajectory(input: &str) -> Option<Trajectory> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();

    if let Some(captures) = re.captures(input) {
        // Parse `p` coordinates
        let px = captures[1].parse::<i32>().unwrap();
        let py = captures[2].parse::<i32>().unwrap();
        let p = IVec2::new(px, py);

        // Parse `v` coordinates
        let vx = captures[3].parse::<i32>().unwrap();
        let vy = captures[4].parse::<i32>().unwrap();
        let v = IVec2::new(vx, vy);

        Some(Trajectory::new(p, v))
    } else {
        None
    }
}

#[allow(dead_code)]
fn draw_grid(trajectories: &[Trajectory], extent: &IVec2) {
    for y in 0..extent.y {
        for x in 0..extent.x {
            let p: IVec2 = IVec2::new(x, y);

            if trajectories.iter().any(|t| t.p == p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn max_contiguous_in_rows(vec: Vec<IVec2>) -> HashMap<i32, usize> {
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();

    // Group elements by row (y coordinate)
    for point in vec {
        rows.entry(point.y).or_default().push(point.x);
    }

    let mut result: HashMap<i32, usize> = HashMap::new();

    // For each row, find the max contiguous elements
    for (y, mut x_values) in rows {
        // Sort the x values
        x_values.sort_unstable();

        // Find the maximum number of contiguous x values
        let mut max_contiguous = 1;
        let mut current_contiguous = 1;

        for i in 1..x_values.len() {
            if x_values[i] == x_values[i - 1] + 1 {
                current_contiguous += 1;
                max_contiguous = max_contiguous.max(current_contiguous);
            } else {
                current_contiguous = 1;
            }
        }

        result.insert(y, max_contiguous);
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut trajectories: Vec<Trajectory> = input.lines().filter_map(parse_trajectory).collect();

    let extent = if cfg!(test) {
        IVec2::new(11, 7)
    } else {
        IVec2::new(101, 103)
    };

    let extent_x_half = extent.x / 2;
    let extent_y_half = extent.y / 2;

    for _ in 0..100 {
        for trajectory in trajectories.iter_mut() {
            trajectory.step(&extent);
        }
    }

    let quadrant1 = trajectories
        .iter()
        .filter(|t| t.p.x < extent_x_half && t.p.y < extent_y_half)
        .count();
    let quadrant2 = trajectories
        .iter()
        .filter(|t| t.p.x > extent_x_half && t.p.y < extent_y_half)
        .count();
    let quadrant3 = trajectories
        .iter()
        .filter(|t| t.p.x < extent_x_half && t.p.y > extent_y_half)
        .count();
    let quadrant4 = trajectories
        .iter()
        .filter(|t| t.p.x > extent_x_half && t.p.y > extent_y_half)
        .count();

    Some(quadrant1 as u32 * quadrant2 as u32 * quadrant3 as u32 * quadrant4 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut trajectories: Vec<Trajectory> = input.lines().filter_map(parse_trajectory).collect();

    let extent = if cfg!(test) {
        IVec2::new(11, 7)
    } else {
        IVec2::new(101, 103)
    };

    let mut loops = 1;

    loop {
        for trajectory in trajectories.iter_mut() {
            trajectory.step(&extent);
        }

        let points: Vec<IVec2> = trajectories.iter().map(|t| t.p).collect();
        let row_contiguous_count = max_contiguous_in_rows(points);

        if row_contiguous_count.values().any(|&v| v > 10) {
            //draw_grid(&trajectories, &extent);
            break;
        }

        loops += 1;
    }

    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
