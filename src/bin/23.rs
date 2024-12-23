use std::collections::HashMap;

use rayon::vec;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut network: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (pc1, pc2) = line.split_once('-').unwrap();

        if !network.contains_key(pc1) {
            network.insert(pc1, Vec::new());
            network.get_mut(pc1).unwrap().push(pc2);
        } else {
            network.get_mut(pc1).unwrap().push(pc2);
        }

        if !network.contains_key(pc2) {
            network.insert(pc2, Vec::new());
            network.get_mut(pc2).unwrap().push(pc1);
        } else {
            network.get_mut(pc2).unwrap().push(pc1);
        }
    }

    let mut interconnected: Vec<Vec<&str>> = Vec::new();

    for (pc, connected) in network.iter() {
        // loop through all combinations of two connected computers and see if they are connected with each other
        for i in 0..connected.len() {
            for j in i + 1..connected.len() {
                if network.get(connected[i]).unwrap().contains(&connected[j]) {
                    let mut cur = vec![*pc, connected[i], connected[j]];

                    cur.sort_by(|a, b| a.cmp(b));

                    if cur[0].chars().nth(0).unwrap() == 't'
                        || cur[1].chars().nth(0).unwrap() == 't'
                        || cur[2].chars().nth(0).unwrap() == 't'
                    {
                        if !interconnected.contains(&cur) {
                            interconnected.push(cur);
                        }
                    }
                }
            }
        }
    }

    Some(interconnected.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut network: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (pc1, pc2) = line.split_once('-').unwrap();

        if !network.contains_key(pc1) {
            network.insert(pc1, Vec::new());
            network.get_mut(pc1).unwrap().push(pc2);
        } else {
            network.get_mut(pc1).unwrap().push(pc2);
        }

        if !network.contains_key(pc2) {
            network.insert(pc2, Vec::new());
            network.get_mut(pc2).unwrap().push(pc1);
        } else {
            network.get_mut(pc2).unwrap().push(pc1);
        }
    }

    let mut parties: Vec<Vec<&str>> = Vec::new();

    for (pc, connections) in network.iter() {
        let mut party = vec![*pc];
        // loop through all combinations of two connected computers and see if they are connected with each other
        for i in 0..connections.len() {
            if party
                .iter()
                .all(|&x| network.get(connections[i]).unwrap().contains(&x))
            {
                party.push(connections[i]);
            }
        }

        party.sort_by(|a, b| a.cmp(b));

        if !parties.contains(&party) {
            parties.push(party);
        }
    }

    parties.sort_by(|a, b| a.len().cmp(&b.len()));

    let result = parties.last().unwrap().join(",");

    Some(result)
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
