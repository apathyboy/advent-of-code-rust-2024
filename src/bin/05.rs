advent_of_code::solution!(5);

struct PageOrderingRule {
    rule1: u32,
    rule2: u32,
}

impl PageOrderingRule {
    fn new(rule1: u32, rule2: u32) -> Self {
        Self { rule1, rule2 }
    }

    fn is_valid(&self, update: &Update) -> bool {
        if !update.contains(&self.rule1) || !update.contains(&self.rule2) {
            return true;
        }

        let rule1_position = update.iter().position(|&x| x == self.rule1).unwrap();
        let rule2_position = update.iter().position(|&x| x == self.rule2).unwrap();

        rule1_position < rule2_position
    }
}

type Update = Vec<u32>;

fn mid(update: &Update) -> u32 {
    let mid = update.len() / 2;
    update[mid]
}

fn reorder_update(update: &[u32], rules: &[PageOrderingRule]) -> Update {
    let mut new_update = update.to_vec();

    let mut swapped = true;
    while swapped {
        swapped = false;
        for rule in rules {
            if !rule.is_valid(&new_update) {
                let rule1_position = new_update.iter().position(|&x| x == rule.rule1).unwrap();
                let rule2_position = new_update.iter().position(|&x| x == rule.rule2).unwrap();

                new_update.swap(rule1_position, rule2_position);
                swapped = true;
            }
        }
    }

    new_update
}

fn parse(input: &str) -> (Vec<PageOrderingRule>, Vec<Update>) {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| {
            let (rule1, rule2) = line.split_once("|").unwrap();
            PageOrderingRule::new(rule1.parse().unwrap(), rule2.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let updates: Vec<Update> = updates_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);

    let result = updates
        .iter()
        .filter(|update| rules.iter().all(|rule| rule.is_valid(update)))
        .map(|update| mid(update))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);

    // Reorder and calculate the mid values of invalid updates
    let result: u32 = updates
        .iter()
        .filter(|update| rules.iter().any(|rule| !rule.is_valid(update)))
        .map(|update| reorder_update(update, &rules))
        .map(|reordered| mid(&reordered))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
