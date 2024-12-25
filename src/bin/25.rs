advent_of_code::solution!(25);

#[derive(Debug)]
enum SchematicType {
    Key,
    Lock,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in input.split("\n\n") {
        let schematic_type = match schematic.lines().next().unwrap().chars().next().unwrap() {
            '#' => SchematicType::Lock,
            '.' => SchematicType::Key,
            _ => panic!("Invalid schematic type"),
        };

        let mut columns = Vec::new();

        for i in 0..5 {
            let column_height = schematic
                .lines()
                .filter(|l| l.chars().nth(i).unwrap() == '#')
                .count()
                - 1;

            columns.push(column_height);
        }

        match schematic_type {
            SchematicType::Lock => locks.push(columns),
            SchematicType::Key => keys.push(columns),
        }
    }

    let mut fits = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if (0..5).all(|i| lock[i] + key[i] <= 5) {
                fits += 1;
            }
        }
    }

    Some(fits)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
