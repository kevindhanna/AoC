use std::io::Read;

#[derive(Clone)]
enum Action {
    Turn,
    Forward,
    North,
    East,
    South,
    West
}

type Direction = i32;
const NORTH: Direction = 0;
const EAST: Direction = 90;
const SOUTH: Direction = 180;
const WEST: Direction = 270;

#[derive(Clone)]
struct Instruction {
    action: Action,
    count: i32
}

struct Vessel {
    heading: Direction,
    x: i32,
    y: i32
}
impl Vessel {

}


fn main() {
    let contents = read_file("input.txt");
    let instructions = contents.trim()
                               .split("\n")
                               .map(|instruction| {
                                   let v = instruction.chars().collect::<Vec<char>>();
                                   return to_instruction(v);
                               })
                               .collect::<Vec<Instruction>>();

    part_1_test();
    part_2_test();
    let part_1_result = part_1(&instructions);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_2(&instructions);
    println!("part 2: {}", part_2_result);
}

fn part_2(instructions: &Vec<Instruction>) -> i32 {
    let waypoint = Vessel {
        heading: NORTH, // Doesn't matter in this scenario
        x: 10,
        y: 1
    };

    let vessel = tick_2(Vessel{ heading: NORTH, x: 0, y: 0}, waypoint, instructions[..].to_vec());

    let mut x = vessel.x;
    let mut y = vessel.y;
    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }
    x + y
}

fn tick_2(mut vessel: Vessel, mut waypoint: Vessel, instructions: Vec<Instruction>) -> Vessel {
    match instructions[0].action {
        Action::Turn => waypoint = rotate(waypoint, instructions[0].count),
        Action::Forward => vessel = move_to_point(vessel, &waypoint, instructions[0].count),
        Action::North => waypoint = sail(waypoint, NORTH, instructions[0].count),
        Action::East => waypoint = sail(waypoint, EAST, instructions[0].count),
        Action::South => waypoint = sail(waypoint, SOUTH, instructions[0].count),
        Action::West => waypoint = sail(waypoint, WEST, instructions[0].count),
    };

    if instructions.len() == 1 {
        return vessel;
    }
    tick_2(vessel, waypoint, instructions[1..].to_vec())
}

fn move_to_point(mut vessel: Vessel, waypoint: &Vessel, count: i32) -> Vessel {
    vessel.x += waypoint.x * count;
    vessel.y += waypoint.y * count;
    vessel
}

fn rotate(mut waypoint: Vessel, mut degrees: i32) -> Vessel {
    if degrees > 0 {
        degrees -= 90;
        let new_y = waypoint.x * -1;
        waypoint.x = waypoint.y;
        waypoint.y = new_y;
    } else {
        degrees += 90;
        let new_x = waypoint.y * -1;
        waypoint.y = waypoint.x;
        waypoint.x = new_x;
    }
    if degrees == 0 {
        return waypoint;
    }
    rotate(waypoint, degrees)
}

fn part_1(instructions: &Vec<Instruction>) -> i32 {
    let vessel = tick_1(Vessel{ heading: EAST, x: 0, y: 0}, instructions[..].to_vec());

    let mut x = vessel.x;
    let mut y = vessel.y;
    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }
    x + y
}

fn tick_1(mut vessel: Vessel, instructions: Vec<Instruction>) -> Vessel {
    vessel = match instructions[0].action {
        Action::Turn => turn(vessel, instructions[0].count),
        Action::Forward => {
            let heading = vessel.heading;
            sail(vessel, heading, instructions[0].count)
        }
        Action::North => sail(vessel, NORTH, instructions[0].count),
        Action::East => sail(vessel, EAST, instructions[0].count),
        Action::South => sail(vessel, SOUTH, instructions[0].count),
        Action::West => sail(vessel, WEST, instructions[0].count),
    };

    if instructions.len() == 1 {
        return vessel;
    }
    tick_1(vessel, instructions[1..].to_vec())
}

fn sail(mut vessel: Vessel, heading: Direction, count: i32) -> Vessel {
    match heading {
        NORTH => vessel.y += count,
        SOUTH => vessel.y -= count,
        EAST => vessel.x += count,
        WEST => vessel.x -= count,
        _ => panic!("where are we going?")
    }
    vessel
}

fn turn(mut vessel: Vessel, change: i32) -> Vessel {
    vessel.heading = (vessel.heading + change + 360) % 360;
    vessel
}


fn to_instruction(i: Vec<char>) -> Instruction {
    let count = i[1..].into_iter().collect::<String>().parse::<i32>().unwrap();
    match i[0] {
        'N' => Instruction{ action: Action::North, count},
        'E' => Instruction{ action: Action::East, count},
        'S' => Instruction{ action: Action::South, count},
        'W' => Instruction{ action: Action::West, count},
        'L' => Instruction { action: Action::Turn, count: count * -1 },
        'R' => Instruction { action: Action::Turn, count },
        'F' => Instruction { action: Action::Forward, count },
        _ => panic!("Uh oh?")
    }
}

fn part_2_test() {
    let instructions = vec![
        to_instruction("F10".chars().collect::<Vec<char>>()),
        to_instruction("N3".chars().collect::<Vec<char>>()),
        to_instruction("F7".chars().collect::<Vec<char>>()),
        to_instruction("R90".chars().collect::<Vec<char>>()),
        to_instruction("F11".chars().collect::<Vec<char>>()),
    ];
    assert_eq!(part_2(&instructions), 286);
}

fn part_1_test() {
    let instructions = vec![
        to_instruction("F10".chars().collect::<Vec<char>>()),
        to_instruction("N3".chars().collect::<Vec<char>>()),
        to_instruction("F7".chars().collect::<Vec<char>>()),
        to_instruction("R90".chars().collect::<Vec<char>>()),
        to_instruction("F11".chars().collect::<Vec<char>>()),
    ];
    assert_eq!(part_1(&instructions), 25);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
