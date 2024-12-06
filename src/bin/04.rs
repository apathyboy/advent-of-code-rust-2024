advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid.first()?.len();

    let mut counter = 0;

    let directions: &[(isize, isize)] = &[
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, -1), // up-left
        (-1, 1),  // up-right
    ];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'X' {
                for &(dy, dx) in directions {
                    if (0..4).all(|i| {
                        let ny = y as isize + dy * i;
                        let nx = x as isize + dx * i;
                        ny >= 0
                            && nx >= 0
                            && (ny as usize) < height
                            && (nx as usize) < width
                            && grid[ny as usize][nx as usize]
                                == "XMAS".chars().nth(i as usize).unwrap()
                    }) {
                        counter += 1;
                    }
                }
            }
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid.first()?.len();

    let mut counter = 0;

    for y in 0..height - 2 {
        for x in 0..width - 2 {
            let down_right: String = (0..3).map(|i| grid[y + i][x + i]).collect();
            let down_left: String = (0..3).map(|i| grid[y + i][x + 2 - i]).collect();

            if (down_right == "MAS" || down_right == "SAM")
                && (down_left == "MAS" || down_left == "SAM")
            {
                counter += 1;
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
