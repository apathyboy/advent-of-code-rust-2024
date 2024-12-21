use glam::IVec2;
use memoize::memoize;

advent_of_code::solution!(21);
fn numpad(key: char) -> IVec2 {
    match key {
        '7' => IVec2::new(0, 0),
        '8' => IVec2::new(0, 1),
        '9' => IVec2::new(0, 2),
        '4' => IVec2::new(1, 0),
        '5' => IVec2::new(1, 1),
        '6' => IVec2::new(1, 2),
        '1' => IVec2::new(2, 0),
        '2' => IVec2::new(2, 1),
        '3' => IVec2::new(2, 2),
        '0' => IVec2::new(3, 1),
        'A' => IVec2::new(3, 2),
        _ => panic!(),
    }
}

fn arrowpad(key: char) -> IVec2 {
    match key {
        '^' => IVec2::new(0, 1),
        'A' => IVec2::new(0, 2),
        '<' => IVec2::new(1, 0),
        'v' => IVec2::new(1, 1),
        '>' => IVec2::new(1, 2),
        _ => panic!(),
    }
}

#[memoize]
fn do_arrows(i: i32, j: i32, steps: usize, h_first: bool) -> usize {
    let (ii, jj) = (i.unsigned_abs() as usize, j.unsigned_abs() as usize);
    let mut chunk = vec![if i > 0 { '^' } else { 'v' }; ii];
    chunk.extend(vec![if j > 0 { '<' } else { '>' }; jj]);

    if h_first {
        chunk.reverse();
    }

    chunk.push('A');

    if steps == 0 {
        chunk.len()
    } else {
        let mut loc = arrowpad('A');

        chunk
            .into_iter()
            .map(|c| {
                let n = arrowpad(c);
                let p = loc;
                loc = n;
                let d = IVec2::new(p.x - n.x, p.y - n.y);
                if d.x == 0 || d.y == 0 {
                    // straight line, search only once, order is irrelevant
                    do_arrows(d.x, d.y, steps - 1, false)
                } else if n == IVec2::new(1, 0) && p.x == 0 {
                    // must search down first
                    do_arrows(d.x, d.y, steps - 1, false)
                } else if p == IVec2::new(1, 0) && n.x == 0 {
                    // must search horiz first
                    do_arrows(d.x, d.y, steps - 1, true)
                } else {
                    // can search in either order
                    std::cmp::min(
                        do_arrows(d.x, d.y, steps - 1, false),
                        do_arrows(d.x, d.y, steps - 1, true),
                    )
                }
            })
            .sum()
    }
}

fn enter_sequence(sequence: &str, steps: usize) -> usize {
    let mut loc = numpad('A');

    sequence
        .chars()
        .map(|c| {
            // either move horizontally or vertically first
            // in some cases only one will be valid...
            let n = numpad(c);
            let p = loc;
            let d = IVec2::new(loc.x - n.x, loc.y - n.y);
            loc = n;
            if p.x == 3 && n.y == 0 {
                // must move up first
                do_arrows(d.x, d.y, steps, false)
            } else if p.y == 0 && n.x == 3 {
                // must move right first
                do_arrows(d.x, d.y, steps, true)
            } else {
                // move in either direction
                std::cmp::min(
                    do_arrows(d.x, d.y, steps, true),
                    do_arrows(d.x, d.y, steps, false),
                )
            }
        })
        .sum::<usize>()
        * sequence[0..3].parse::<usize>().unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    memoized_flush_do_arrows();
    Some(input.lines().map(|l| enter_sequence(l, 2)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    memoized_flush_do_arrows();
    Some(input.lines().map(|l| enter_sequence(l, 25)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
