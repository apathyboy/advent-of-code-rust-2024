use glam::IVec2;
use once_cell::sync::Lazy;
use regex::Regex;

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

static TRAJECTORY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap());

fn parse_trajectory(input: &str) -> Option<Trajectory> {
    TRAJECTORY_REGEX.captures(input).and_then(|captures| {
        let parse_coord = |idx: usize| captures.get(idx)?.as_str().parse::<i32>().ok();
        Some(Trajectory::new(
            IVec2::new(parse_coord(1)?, parse_coord(2)?),
            IVec2::new(parse_coord(3)?, parse_coord(4)?),
        ))
    })
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

fn max_contiguous_in_rows(trajectories: &[Trajectory], max: u32) -> bool {
    // Sort the points by (y, x) to process rows and contiguous x-values in order
    let mut sorted_points: Vec<IVec2> = trajectories.iter().map(|t| t.p).collect();
    sorted_points.sort_unstable_by_key(|p| (p.y, p.x));

    // Variables to track current row and contiguous x-values
    let mut current_y = None;
    let mut prev_x = None;
    let mut current_contiguous = 0;

    for point in sorted_points {
        match current_y {
            Some(y) if y == point.y => {
                // Same row, check if x is contiguous
                if let Some(prev_x) = prev_x {
                    if point.x == prev_x + 1 {
                        current_contiguous += 1;
                        if current_contiguous >= max {
                            return true;
                        }
                    } else {
                        current_contiguous = 1;
                    }
                } else {
                    current_contiguous = 1;
                }
                prev_x = Some(point.x);
            }
            _ => {
                // New row, reset counters
                current_y = Some(point.y);
                prev_x = Some(point.x);
                current_contiguous = 1;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut trajectories: Vec<Trajectory> = input.lines().filter_map(parse_trajectory).collect();

    let extent = if cfg!(test) {
        IVec2::new(11, 7)
    } else {
        IVec2::new(101, 103)
    };

    for _ in 0..100 {
        trajectories
            .iter_mut()
            .for_each(|trajectory| trajectory.step(&extent));
    }

    let (extent_x_half, extent_y_half) = (extent.x / 2, extent.y / 2);

    let quadrants = trajectories.iter().fold([0; 4], |mut counts, t| {
        let (x, y) = (t.p.x, t.p.y);

        // Skip cases where x or y are exactly on the half-extents
        if x == extent_x_half || y == extent_y_half {
            return counts;
        }

        let idx = match (x < extent_x_half, y < extent_y_half) {
            (true, true) => 0,   // Quadrant 1
            (false, true) => 1,  // Quadrant 2
            (true, false) => 2,  // Quadrant 3
            (false, false) => 3, // Quadrant 4
        };
        counts[idx] += 1;
        counts
    });

    Some(quadrants.iter().map(|&q| q as u32).product())
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
        if max_contiguous_in_rows(&trajectories, 10) {
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
