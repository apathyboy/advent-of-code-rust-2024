advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Computer {
    registers: [u64; 3],
    ip: usize,
    program: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn new(program: Vec<u64>, register_a: u64, register_b: u64, register_c: u64) -> Self {
        let mut registers = [0; 3];
        registers[0] = register_a;
        registers[1] = register_b;
        registers[2] = register_c;
        Self {
            registers,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            self.run_cycle();
        }
    }

    fn run_cycle(&mut self) {
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        self.execute(opcode, operand);
    }

    fn operand_value(&self, operand: u64) -> u64 {
        match operand {
            1..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Unknown operand: {}", operand),
        }
    }

    fn division(&mut self, operand: u64) -> u64 {
        let combo_operand = self.operand_value(operand);
        let result = self.registers[0] as f64 / 2_u64.pow(combo_operand as u32) as f64;

        result.floor() as u64
    }

    fn execute(&mut self, opcode: u64, operand: u64) {
        match opcode {
            0 => {
                self.registers[0] = self.division(operand);
                self.ip += 2;
            }
            1 => {
                self.registers[1] ^= operand;
                self.ip += 2;
            }
            2 => {
                let combo_operand = self.operand_value(operand);
                self.registers[1] = combo_operand.rem_euclid(8);
                self.ip += 2;
            }
            3 => {
                if self.registers[0] == 0 {
                    self.ip += 2;
                } else {
                    self.ip = operand as usize;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
                self.ip += 2;
            }
            5 => {
                let output = self.operand_value(operand).rem_euclid(8);
                self.output.push(output);
                self.ip += 2;
            }
            6 => {
                self.registers[1] = self.division(operand);
                self.ip += 2;
            }
            7 => {
                self.registers[2] = self.division(operand);
                self.ip += 2;
            }
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }
}

fn search(idx: usize, so_far: u64, expected: &[u64]) -> Option<u64> {
    if idx == expected.len() {
        return Some(so_far);
    }

    let results = (0..8).filter_map(|next_chunk| {
        // Precompute bit shift for efficiency
        let shift_amount = 3 * (expected.len() - idx - 1);
        let candidate = so_far + ((next_chunk as u64) << shift_amount);
        let scaled_candidate = candidate >> shift_amount;

        // Bitwise comparison check
        if next_chunk ^ ((scaled_candidate >> (next_chunk ^ 7)) & 7) as u8 != expected[idx] as u8 {
            return None;
        }

        // Recurse and return the result
        search(idx + 1, candidate, expected)
    });

    // Return the minimum result found, if any
    results.min()
}

fn parse(input: &str) -> Option<Computer> {
    let mut lines = input.lines();
    let register_a = lines.next()?[12..].parse().ok()?;
    let register_b = lines.next()?[12..].parse().ok()?;
    let register_c = lines.next()?[12..].parse().ok()?;

    // read an empty line
    lines.next()?;

    let program = lines
        .next()?
        .split_whitespace()
        .nth(1)?
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    Some(Computer::new(program, register_a, register_b, register_c))
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = parse(input)?;

    computer.run();

    Some(
        computer
            .output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = parse(input)?;
    let program_rev = computer.program.iter().rev().cloned().collect::<Vec<_>>();
    search(0, 0, &program_rev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }
}
