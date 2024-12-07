use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    total: u64,
    nums: Vec<u64>,
}

impl Equation {
    fn new() -> Self {
        Self {
            total: 0,
            nums: Vec::new(),
        }
    }
}

fn generate_permutations(
    permutations_cache: &mut HashMap<usize, Vec<Vec<char>>>,
    chars: &[char],
    length: usize,
) -> Vec<Vec<char>> {
    if let Some(results) = permutations_cache.get(&length) {
        return results.clone(); // Return a cloned result to avoid borrowing issues.
    }

    let mut results = Vec::new();

    fn backtrack(
        current: &mut Vec<char>,
        length: usize,
        chars: &[char],
        results: &mut Vec<Vec<char>>,
    ) {
        if current.len() == length {
            results.push(current.clone());
            return;
        }
        for &ch in chars {
            current.push(ch);
            backtrack(current, length, chars, results);
            current.pop();
        }
    }

    backtrack(&mut Vec::new(), length, chars, &mut results);

    // Insert into the cache after computation.
    permutations_cache.insert(length, results.clone());

    results
}

fn try_evaluate(
    permutations_cache: &mut HashMap<usize, Vec<Vec<char>>>,
    chars: &[char],
    equation: Equation,
) -> Option<u64> {
    let test_opers = generate_permutations(permutations_cache, chars, equation.nums.len() - 1);

    for oper in test_opers {
        let mut result = equation.nums[0];

        for (op, num) in oper.iter().zip(equation.nums.iter().skip(1)) {
            match op {
                '+' => result += num,
                '*' => result *= num,
                '|' => result = result * 10u64.pow(num.to_string().len() as u32) + num,
                _ => panic!("Invalid operator"),
            }
        }

        if result == equation.total {
            return Some(result);
        }
    }

    None
}

fn parse_equation(line: &str) -> Equation {
    let mut equation = Equation::new();

    let (total, rest) = line.split_once(": ").unwrap();

    equation.total = total.parse().unwrap();

    for num in rest.split(' ') {
        if let Ok(num) = num.parse() {
            equation.nums.push(num);
        }
    }

    equation
}

pub fn part_one(input: &str) -> Option<u64> {
    let opers = vec!['*', '+'];

    let mut permutations_cache: HashMap<usize, Vec<Vec<char>>> = HashMap::new();

    let calibration_result = input
        .lines()
        .filter_map(|line| {
            let equation = parse_equation(line);
            try_evaluate(&mut permutations_cache, &opers, equation)
        })
        .sum();

    Some(calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let opers = vec!['*', '+', '|'];

    let mut permutations_cache: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
    let calibration_result = input
        .lines()
        .filter_map(|line| {
            let equation = parse_equation(line);
            try_evaluate(&mut permutations_cache, &opers, equation)
        })
        .sum();

    Some(calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
