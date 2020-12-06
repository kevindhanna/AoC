use std::io::Read;

struct Seat {
    row: u32,
    col: u32,
}

#[derive(Copy,Clone)]
enum Instruction {
    High,
    Low,
}

#[derive(Copy,Clone)]
struct Ticket {
    id: u32,
    row: u32,
    col: u32,
}

const ROWS: u32 = 127;
const COLS: u32 = 7;

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn part_1(tickets: &Vec<&str>) -> u32 {
    let mut max: u32 = 0;
    for string in tickets {
        let ticket: Ticket = build_ticket(string);
        if max < ticket.id {
            max = ticket.id;
        }
    }
    max
}

fn part_2(tickets: &Vec<&str>) -> u32 {
    let mut seats: Vec<Vec<Option<Ticket>>> = Vec::new();
    for i in 0..=ROWS {
        seats.push(vec![None; 8]);
    }

    for string in tickets {
        let ticket: Ticket = build_ticket(string);
        seats[ticket.row as usize][ticket.col as usize] = Some(ticket);
    }
    let mut my_ticket: Ticket = Ticket{row: 0, col: 0, id: 0};
    for (i, row) in seats.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if j > 6 || j < 1 {
                continue;
            }
            match seat {
                Some(val) => continue,
                None => {
                    if row[j + 1].is_some() &&
                       row[j - 1].is_some() {
                           my_ticket = Ticket{
                               row: i as u32,
                               col: j as u32,
                               id: (i as u32 * 8) + j as u32
                           };
                    }
                }
            }
        }
    }
    if my_ticket.id == 0 {
        panic!("uh oh");
    }
    my_ticket.id
}

fn build_ticket(ticket: &str) -> Ticket {
    let mut instructions: Vec<Instruction> = Vec::new();
    for letter in ticket.chars() {
        match letter {
            'R' => instructions.push(Instruction::High),
            'B' => instructions.push(Instruction::High),
            'F' => instructions.push(Instruction::Low),
            'L' => instructions.push(Instruction::Low),
            _ => panic!("Shouldn't get here"),
        }
    }
    let row: u32 = calculate(instructions[..7].to_vec(), ROWS);
    let col: u32 = calculate(instructions[7..].to_vec(), COLS);
    let id: u32 = (row * 8) + col;
    Ticket{row, col, id}
}

fn calculate(instructions: Vec<Instruction>, mut upper: u32) -> u32 {
    let mut lower = 0;
    let last_i = instructions.len() - 1;

    for (i,x) in instructions[..last_i].iter().enumerate() {
        match x {
            Instruction::High => lower += 1 << last_i-i,
            Instruction::Low => upper -= 1 << last_i-i,
        }
    }
    match instructions[last_i] {
        Instruction::High => upper,
        Instruction::Low => lower,
    }
}

fn part_1_tests() {
    let string1 = "FBFBBFFRLR";
    let string2 = "BFFFBBFRRR";
    let string3 = "FFFBBBFRRR";
    let string4 = "BBFFBBFRLL";

    assert_eq!(part_1(&vec!(string1)), 357);
    assert_eq!(part_1(&vec!(string2)), 567);
    assert_eq!(part_1(&vec!(string3)), 119);
    assert_eq!(part_1(&vec!(string4)), 820);
}

fn main() {
    let contents = read_file("input.txt");
    let tickets: Vec<&str> = contents
                     .trim()
                     .split("\n")
                     .map(|ticket| ticket.trim())
                     .collect();
    // part_1_tests();

    let part_1_result = part_1(&tickets);
    let part_2_result = part_2(&tickets);

    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);
}
