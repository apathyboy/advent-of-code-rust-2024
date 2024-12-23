use std::collections::HashMap;

advent_of_code::solution!(23);

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut network: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (pc1, pc2) = line.split_once('-').unwrap();

        network.entry(pc1).or_default().push(pc2);
        network.entry(pc2).or_default().push(pc1);
    }

    network
}

pub fn part_one(input: &str) -> Option<u32> {
    let network = parse(input);

    let mut interconnected: Vec<Vec<&str>> = Vec::new();

    // Check all connections for interconnected triples
    for (pc, connected) in &network {
        for (i, &node1) in connected.iter().enumerate() {
            for &node2 in &connected[i + 1..] {
                if let Some(connections) = network.get(node1) {
                    if connections.contains(&node2) {
                        let mut cur = vec![*pc, node1, node2];
                        cur.sort_unstable();

                        // Check if any name starts with 't'
                        if cur.iter().any(|&name| name.starts_with('t')) {
                            interconnected.push(cur);
                        }
                    }
                }
            }
        }
    }

    // Remove duplicate groups
    interconnected.sort();
    interconnected.dedup();

    Some(interconnected.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let network = parse(input);

    let mut parties: Vec<Vec<&str>> = Vec::new();

    for (pc, connections) in network.iter() {
        let mut party = vec![*pc];
        // loop through all combinations of two connected computers and see if they are connected with each other
        for &connection in connections {
            if party
                .iter()
                .all(|&x| network.get(connection).unwrap().contains(&x))
            {
                party.push(connection);
            }
        }

        party.sort_unstable();

        if !parties.contains(&party) {
            parties.push(party);
        }
    }

    // Find the largest party
    let largest_party = parties.into_iter().max_by_key(|party| party.len())?;

    Some(largest_party.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
