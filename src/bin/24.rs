use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

advent_of_code::solution!(24);

static GATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-z0-9]{3})\s(XOR|OR|AND)\s([a-z0-9]{3})\s->\s([a-z0-9]{3})").unwrap()
});

#[derive(Debug)]
struct Wire {
    label: String,
    output: Option<u32>,
    gates: Vec<Weak<RefCell<Gate>>>,
}

impl Wire {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            output: None,
            gates: Vec::new(),
        }
    }

    fn set_input(&mut self, input: u32) {
        self.output = Some(input);

        for weak_gate in &self.gates {
            if let Some(gate) = weak_gate.upgrade() {
                gate.borrow_mut().set_input(input);
            }
        }
    }
}

// Implement PartialEq for Wire
impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.output == other.output
    }
}

// Implement Eq for Wire
impl Eq for Wire {}

#[derive(Debug, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    inputs: Vec<u32>,
    wire_out: Rc<RefCell<Wire>>,
}

impl Gate {
    fn new(gate_type: GateType, wire_out: Rc<RefCell<Wire>>) -> Self {
        Self {
            gate_type,
            inputs: Vec::new(),
            wire_out,
        }
    }

    fn set_input(&mut self, input: u32) {
        self.inputs.push(input);

        if self.inputs.len() == 2 {
            self.compute_output();
        }
    }

    fn compute_output(&self) {
        let output = match self.gate_type {
            GateType::And => self.inputs[0] & self.inputs[1],
            GateType::Or => self.inputs[0] | self.inputs[1],
            GateType::Xor => self.inputs[0] ^ self.inputs[1],
        };

        self.wire_out.borrow_mut().set_input(output);
    }
}

fn binary_to_decimal(binary: Vec<u32>) -> u128 {
    binary
        .iter()
        .enumerate()
        .map(|(index, &bit)| (bit as u128) * (1u128 << index))
        .sum()
}

pub fn part_one(input: &str) -> Option<u128> {
    let (initial_vals_str, gate_map_str) = input.split_once("\n\n")?;
    let initial_vals: Vec<(String, u32)> = initial_vals_str
        .lines()
        .map(|line| {
            let (wire_str, val_str) = line.split_once(": ").unwrap();
            let val = val_str.parse().unwrap();
            (wire_str.to_string(), val)
        })
        .collect();

    let mut wires: Vec<Rc<RefCell<Wire>>> = Vec::new();
    let mut gates: Vec<Rc<RefCell<Gate>>> = Vec::new();

    for gate_str in gate_map_str.lines() {
        GATE_REGEX.captures_iter(gate_str).for_each(|cap| {
            let wire_out_label = cap[4].to_string();
            let wire_out = wires
                .iter()
                .find(|wire| wire.borrow().label == wire_out_label)
                .cloned()
                .unwrap_or_else(|| {
                    let new_wire = Rc::new(RefCell::new(Wire::new(&wire_out_label)));
                    wires.push(Rc::clone(&new_wire));
                    new_wire
                });

            let gate_type = match &cap[2] {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("Invalid gate type"),
            };

            let gate = Rc::new(RefCell::new(Gate::new(gate_type, Rc::clone(&wire_out))));
            gates.push(Rc::clone(&gate));

            let wire_in1_label = cap[1].to_string();
            let wire_in2_label = cap[3].to_string();

            for wire_label in [wire_in1_label, wire_in2_label] {
                let wire = wires
                    .iter()
                    .find(|wire| wire.borrow().label == wire_label)
                    .cloned()
                    .unwrap_or_else(|| {
                        let new_wire = Rc::new(RefCell::new(Wire::new(&wire_label)));
                        wires.push(Rc::clone(&new_wire));
                        new_wire
                    });

                wire.borrow_mut().gates.push(Rc::downgrade(&gate));
            }
        });
    }

    for (wire_label, val) in initial_vals {
        let wire = wires
            .iter()
            .find(|wire| wire.borrow().label == wire_label)
            .unwrap();

        wire.borrow_mut().set_input(val);
    }

    // find all wire starting with z
    let mut wire_z: Vec<(String, u32)> = wires
        .iter()
        .filter(|wire| wire.borrow().label.starts_with('z'))
        .map(|wire| {
            let wire = wire.borrow();
            (wire.label.clone(), wire.output.unwrap())
        })
        .collect();

    // sort wire_z by the first item in each tuple and then map them to their output
    wire_z.sort_by(|a, b| a.0.cmp(&b.0));

    Some(binary_to_decimal(
        wire_z.iter().map(|(_, output)| *output).collect(),
    ))
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, gate_connections) = input.split_once("\n\n")?;

    let gate_connections = gate_connections
        .lines()
        .map(|line| {
            let caps = GATE_REGEX.captures(line).unwrap();
            let (_, s) = caps.extract::<4>();
            s
        })
        .collect::<Vec<_>>();

    let mut wire_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();

    for &[lhs, op, rhs, ret] in &gate_connections {
        wire_map.entry(lhs).or_default().push((op, ret));
        wire_map.entry(rhs).or_default().push((op, ret));
    }

    let mut wrong_outputs = vec![];
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        let chained_ops = wire_map.get(&ret);
        let chained_ops_contain = |op| chained_ops.is_some_and(|v| v.iter().any(|(o, _)| o == &op));

        let has_chained_xor = chained_ops_contain("XOR");
        let has_chained_and = chained_ops_contain("AND");
        let has_chained_or = chained_ops_contain("OR");
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = ret.starts_with('z');
        let outputs_last_bit = ret == "z45";

        let valid = match op {
            "XOR" => {
                (!takes_input_bit && outputs_bit)
                    || (takes_input_bit && has_chained_xor)
                    || (takes_first_input && outputs_bit)
            }
            "OR" => outputs_last_bit || (has_chained_and && has_chained_xor),
            "AND" => has_chained_or || takes_first_input,
            _ => unreachable!(),
        };

        if !valid {
            wrong_outputs.push(ret);
        }
    }

    wrong_outputs.sort();

    Some(wrong_outputs.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 4)]
    #[case(2, 2024)]
    fn test_part_one(#[case] part: u8, #[case] expected: u128) {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, part,
        ));
        assert_eq!(result, Some(expected));
    }
}
