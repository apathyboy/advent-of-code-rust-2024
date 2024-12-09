advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Vec::new();

    input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| c.to_digit(10).map(|n| (i, n)))
        .for_each(|(i, n)| {
            if i % 2 == 0 {
                disk.extend(vec![(i / 2) as i64; n as usize]);
            } else {
                disk.extend(vec![-1; n as usize]);
            }
        });

    // Swap the first `-1` with the last non-`-1`
    let mut last_non_minus_one = disk.len().saturating_sub(1);
    let mut checksum = 0;

    for i in 0..disk.len() {
        if disk[i] == -1 {
            while disk[last_non_minus_one] == -1 && last_non_minus_one > i {
                last_non_minus_one -= 1;
            }
            if last_non_minus_one > i {
                disk.swap(i, last_non_minus_one);
            }
        }

        if disk[i] != -1 {
            checksum += i as i64 * disk[i];
        }
    }

    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut program_id = 0;

    let mut disk = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if !c.is_ascii_digit() {
            continue;
        }

        if i % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                disk.push(program_id);
            }
            program_id += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                disk.push(-1);
            }
        }
    }

    program_id -= 1;

    while program_id > 0 {
        let file_pos = disk.iter().position(|&x| x == program_id).unwrap();
        let file_len = disk.iter().filter(|&&x| x == program_id).count();

        let mut free_space_offset = 0;
        while free_space_offset < file_pos {
            if disk[free_space_offset] == -1 {
                let mut free_space_len = 0;
                while free_space_offset + free_space_len < disk.len()
                    && disk[free_space_offset + free_space_len] == -1
                {
                    free_space_len += 1;
                }

                if free_space_offset + free_space_len > file_pos {
                    break;
                }

                if free_space_len >= file_len {
                    for i in 0..file_len {
                        disk.swap(free_space_offset + i, file_pos + i);
                    }
                    break;
                } else {
                    free_space_offset += free_space_len + 1;
                }
            } else {
                free_space_offset += 1;
            }
        }

        program_id -= 1;
    }

    let mut checksum = 0;

    for (i, id) in disk.iter().enumerate() {
        if disk[i] != -1 {
            checksum += i as i64 * id;
        }
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
