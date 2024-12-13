use glam::I64Vec2;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Game {
    button_a: I64Vec2,
    button_b: I64Vec2,
    prize: I64Vec2,
}

impl Game {
    fn new(button_a: I64Vec2, button_b: I64Vec2, prize: I64Vec2) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn solve(&self, offset: i64) -> Option<(i64, i64)> {
        let pxo = self.prize.x + offset;
        let pyo = self.prize.y + offset;

        let denominator = self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y;
        if denominator == 0 {
            return None;
        }

        let n = (pyo * self.button_a.x - pxo * self.button_a.y) / denominator;

        let m = if self.button_a.x != 0 {
            (pxo - n * self.button_b.x) / self.button_a.x
        } else if self.button_a.y != 0 {
            (pyo - n * self.button_b.y) / self.button_a.y
        } else {
            return None;
        };

        if m >= 0
            && n >= 0
            && m * self.button_a.x + n * self.button_b.x == pxo
            && m * self.button_a.y + n * self.button_b.y == pyo
        {
            Some((m, n))
        } else {
            None
        }
    }
}

fn parse_button(line: &str) -> Option<I64Vec2> {
    let re = regex::Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();

    if let Some(captures) = re.captures(line) {
        let x = captures.get(1).unwrap().as_str().parse().unwrap();
        let y = captures.get(2).unwrap().as_str().parse().unwrap();
        Some(I64Vec2::new(x, y))
    } else {
        None
    }
}

fn parse_prize(line: &str) -> Option<I64Vec2> {
    let re = regex::Regex::new(r"X\=(\d+), Y\=(\d+)").unwrap();

    if let Some(captures) = re.captures(line) {
        let x = captures.get(1).unwrap().as_str().parse().unwrap();
        let y = captures.get(2).unwrap().as_str().parse().unwrap();
        Some(I64Vec2::new(x, y))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let games: Vec<Game> = input
        .split("\n\n")
        .map(|game| {
            let mut lines = game.lines();
            let button_a = parse_button(lines.next().unwrap());
            let button_b = parse_button(lines.next().unwrap());
            let prize = parse_prize(lines.next().unwrap());

            Game::new(button_a.unwrap(), button_b.unwrap(), prize.unwrap())
        })
        .collect();

    let cheapest_wins = games
        .iter()
        .map(|game| {
            if let Some((m, n)) = game.solve(0) {
                m.unsigned_abs() * 3 + n.unsigned_abs()
            } else {
                0
            }
        })
        .sum();

    Some(cheapest_wins)
}

pub fn part_two(input: &str) -> Option<u64> {
    let games: Vec<Game> = input
        .split("\n\n")
        .map(|game| {
            let mut lines = game.lines();
            let button_a = parse_button(lines.next().unwrap());
            let button_b = parse_button(lines.next().unwrap());
            let prize = parse_prize(lines.next().unwrap());

            Game::new(button_a.unwrap(), button_b.unwrap(), prize.unwrap())
        })
        .collect();

    let cheapest_wins = games
        .iter()
        .map(|game| {
            if let Some((m, n)) = game.solve(10_000_000_000_000) {
                m.unsigned_abs() * 3 + n.unsigned_abs()
            } else {
                0
            }
        })
        .sum();

    Some(cheapest_wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
