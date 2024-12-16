use glam::IVec2;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq)]
enum WarehouseObjectType {
    Wall,
    //Empty,
    Box,
}

#[derive(Debug, Clone, Copy)]
struct WarehouseObject {
    pos: IVec2,
    size: IVec2,
    object_type: WarehouseObjectType,
}

#[derive(Debug)]
struct WarehouseMap {
    objects: Vec<WarehouseObject>,
}

impl WarehouseMap {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn insert(&mut self, pos: IVec2, size: IVec2, object_type: WarehouseObjectType) {
        self.objects.push(WarehouseObject {
            pos,
            size,
            object_type,
        });
    }

    fn get(&self, pos: IVec2) -> Option<WarehouseObject> {
        for object in &self.objects {
            // check if the position is inside the object
            if pos.x >= object.pos.x
                && pos.x < object.pos.x + object.size.x
                && pos.y >= object.pos.y
                && pos.y < object.pos.y + object.size.y
            {
                return Some(*object);
            }
        }

        None
    }

    fn get_mut(&mut self, pos: IVec2) -> Option<&mut WarehouseObject> {
        self.objects.iter_mut().find(|object| {
            pos.x >= object.pos.x
                && pos.x < object.pos.x + object.size.x
                && pos.y >= object.pos.y
                && pos.y < object.pos.y + object.size.y
        })
    }

    fn can_push_box(&self, pos: IVec2, direction: IVec2) -> bool {
        let cur_obj = self.get(pos).unwrap();
        let mut npos = cur_obj.pos + direction;
        if direction == IVec2::new(1, 0) && cur_obj.size.x == 2 {
            npos += IVec2::new(1, 0);
        }
        let mut check_positions = vec![npos];

        if (direction == IVec2::new(0, 1) || direction == IVec2::new(0, -1)) && cur_obj.size.x == 2
        {
            check_positions.push(cur_obj.pos + IVec2::new(1, 0) + direction);
        }

        check_positions.iter().all(|pos| {
            if let Some(obj) = self.get(*pos) {
                match obj.object_type {
                    WarehouseObjectType::Wall => false,
                    WarehouseObjectType::Box => self.can_push_box(obj.pos, direction),
                }
            } else {
                true
            }
        })
    }

    // try to push the box in the given direction. if a box is already there, try to push that box as well and so on
    // if there is a wall or the box can't be pushed, return false
    fn push_box(&mut self, pos: IVec2, direction: IVec2) -> bool {
        if !self.can_push_box(pos, direction) {
            return false;
        }

        let cur_obj = self.get(pos).unwrap();
        let mut npos = cur_obj.pos + direction;
        if direction == IVec2::new(1, 0) && cur_obj.size.x == 2 {
            npos += IVec2::new(1, 0);
        }
        let mut move_positions = vec![npos];

        if (direction == IVec2::new(0, 1) || direction == IVec2::new(0, -1)) && cur_obj.size.x == 2
        {
            move_positions.push(cur_obj.pos + IVec2::new(1, 0) + direction);
        }

        for npos in move_positions {
            if let Some(obj) = self.get(npos) {
                if obj.object_type == cur_obj.object_type && obj.pos == cur_obj.pos {
                    continue;
                }

                match obj.object_type {
                    WarehouseObjectType::Wall => {}
                    WarehouseObjectType::Box => {
                        self.push_box(npos, direction);
                    }
                }
            }
        }

        let obj = self.get_mut(pos).unwrap();
        obj.pos += direction;

        true
    }
}

fn parse(input: &str, width: i32) -> (WarehouseMap, Vec<IVec2>, IVec2) {
    let mut map = WarehouseMap::new();
    let mut directions = Vec::new();
    let mut robot = IVec2::new(0, 0);
    let size = IVec2::new(width, 1);

    let (map_input, directions_input) = input.split_once("\n\n").unwrap();

    for (y, line) in map_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i32 * width;
            let y = y as i32;

            let pos = IVec2::new(x, y);

            match c {
                '#' => map.insert(pos, size, WarehouseObjectType::Wall),
                '.' => {}
                'O' => map.insert(pos, size, WarehouseObjectType::Box),
                '@' => {
                    //map.insert(pos, size, WarehouseObjectType::Empty);
                    robot = IVec2::new(x, y);
                }
                _ => panic!("Invalid character"),
            }
        }
    }

    for line in directions_input.lines() {
        for c in line.chars() {
            let direction = match c {
                '^' => IVec2::new(0, -1),
                'v' => IVec2::new(0, 1),
                '<' => IVec2::new(-1, 0),
                '>' => IVec2::new(1, 0),
                _ => panic!("Invalid character"),
            };
            directions.push(direction);
        }
    }

    (map, directions, robot)
}

#[allow(dead_code)]
fn draw_warehouse_map(map: &WarehouseMap, robot: IVec2) {
    let mut min = IVec2::new(i32::MAX, i32::MAX);
    let mut max = IVec2::new(i32::MIN, i32::MIN);

    for object in &map.objects {
        min = min.min(object.pos);
        max = max.max(object.pos + object.size);
    }

    for y in min.y..max.y {
        for x in min.x..max.x {
            let pos = IVec2::new(x, y);

            if let Some(obj) = map.get(pos) {
                match obj.object_type {
                    WarehouseObjectType::Wall => {
                        print!("#");
                    }
                    WarehouseObjectType::Box => {
                        if obj.size.x == 2 {
                            if obj.pos.x == x {
                                print!("[");
                            } else {
                                print!("]");
                            }
                        } else {
                            print!("O");
                        }
                    }
                }
            } else if pos == robot {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn get_move_char_from_direction(direction: IVec2) -> char {
    match direction {
        IVec2 { x: 0, y: -1 } => '^',
        IVec2 { x: 0, y: 1 } => 'v',
        IVec2 { x: -1, y: 0 } => '<',
        IVec2 { x: 1, y: 0 } => '>',
        _ => panic!("Invalid direction"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, directions, mut robot) = parse(input, 1);

    //println!("Initial state:");
    //draw_warehouse_map(&map, robot);
    //println!();

    for direction in directions {
        let npos = robot + direction;

        if let Some(obj) = map.get(npos) {
            match obj.object_type {
                WarehouseObjectType::Wall => {}
                WarehouseObjectType::Box => {
                    if map.push_box(npos, direction) {
                        robot = npos;
                    }
                }
            }
        } else {
            robot = npos;
        }

        //println!("Move: {}", get_move_char_from_direction(direction));
        //draw_warehouse_map(&map, robot);
        //println!();
    }

    // get all boxes from the map and multitply their y by 100 and add to their x, then sum all of them
    let sum: i32 = map
        .objects
        .iter()
        .filter_map(|o| match o.object_type {
            WarehouseObjectType::Box => Some(o.pos.y * 100 + o.pos.x),
            _ => None,
        })
        .sum();

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, directions, mut robot) = parse(input, 2);

    //println!("Initial state:");
    //draw_warehouse_map(&map, robot);
    //println!();

    for direction in directions {
        let npos = robot + direction;
        if let Some(obj) = map.get(npos) {
            match obj.object_type {
                WarehouseObjectType::Wall => {}
                WarehouseObjectType::Box => {
                    if map.push_box(npos, direction) {
                        robot = npos;
                    }
                }
            }
        } else {
            robot = npos;
        }

        //println!("Move: {}", get_move_char_from_direction(direction));
        //draw_warehouse_map(&map, robot);
        //println!();
    }

    // get all boxes from the map and multitply their y by 100 and add to their x, then sum all of them
    let sum: i32 = map
        .objects
        .iter()
        .filter_map(|o| match o.object_type {
            WarehouseObjectType::Box => Some(o.pos.y * 100 + o.pos.x),
            _ => None,
        })
        .sum();

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_complex() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
