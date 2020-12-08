use std::io::Read;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Instruction {
    action: Action,
    count: i32,
}

struct Pointer {
    head: i32,
    accumulator: i32
}

#[derive(Copy, Clone)]
enum Action {
    Acc,
    Jmp,
    Nop
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn build_instruction_set(instructions: Vec<Vec<&str>>) -> Vec<Instruction> {
    instructions.iter().map(|instruction| {
        Instruction {
            action: match instruction[0] {
                "acc" => Action::Acc,
                "jmp" => Action::Jmp,
                "nop" => Action::Nop,
                _ => panic!()
            },
            count: instruction[1].parse::<i32>().unwrap(),
        }
    }).collect::<Vec<Instruction>>()
}

fn clone_set_replace(set: &Vec<Instruction>, index: i32, action: Action) -> Vec<Instruction> {
    let mut new_set = set.clone();
    new_set[index as usize].action = action;
    new_set
}

fn brutus(instruction_set: &Vec<Instruction>) -> Result<i32, Vec<i32>> {
    let mut pointer = Pointer{ head: 0, accumulator: 0 };
    let mut visited: Vec<i32> = Vec::new();
    loop {
        if visited.contains(&pointer.head) {
            return Err(visited);
        }
        let instruction = instruction_set[pointer.head as usize];
        visited.push(pointer.head);
        match instruction.action {
            Action::Acc => {
                pointer.accumulator += instruction.count;
                pointer.head += 1
            },
            Action::Jmp => {
                pointer.head += instruction.count
            },
            Action::Nop => {
                pointer.head += 1;
            },
        }
        if pointer.head as usize == instruction_set.len() {
            break;
        }
    }
    Ok(pointer.accumulator)
}

fn part_1(instruction_set: &Vec<Instruction>) -> i32 {
    let mut pointer = Pointer{ head: 0, accumulator: 0 };
    let mut visited: Vec<i32> = Vec::new();
    loop {
        if visited.contains(&pointer.head) {
            return pointer.accumulator
        }
        let instruction = instruction_set[pointer.head as usize];
        visited.push(pointer.head);
        match instruction.action {
            Action::Acc => {
                pointer.accumulator += instruction.count;
                pointer.head += 1
            },
            Action::Jmp => {
                pointer.head += instruction.count
            },
            Action::Nop => {
                pointer.head += 1;
            },
        }
    }
}

fn part_2_brute(instruction_set: &Vec<Instruction>) -> i32 {
    for (i, instruction) in instruction_set.iter().enumerate() {
        match instruction.action {
            Action::Nop => {
                let new_set = clone_set_replace(instruction_set, i as i32, Action::Jmp);
                match brutus(&new_set) {
                    Ok(val) => return val,
                    Err(_) => continue
                }
            },
            Action::Jmp => {
                let new_set = clone_set_replace(instruction_set, i as i32, Action::Nop);
                match brutus(&new_set) {
                    Ok(val) => return val,
                    Err(_) => continue
                }
            },
            Action::Acc => continue
        }
    }
    panic!("shouldn't get here");
}

fn part_2_not_quite_brute(instruction_set: &Vec<Instruction>) -> i32 {
    if let Err(mut visited) = brutus(instruction_set) {
        while let Some(index) = visited.pop() {
            let instruction = instruction_set[index as usize];
            match instruction.action {
                Action::Nop => {
                    let new_set = clone_set_replace(instruction_set, index, Action::Jmp);
                    match brutus(&new_set) {
                        Ok(val) => return val,
                        Err(_) => continue
                    }
                },
                Action::Jmp => {
                    let new_set = clone_set_replace(instruction_set, index, Action::Nop);
                    match brutus(&new_set) {
                        Ok(val) => return val,
                        Err(_) => continue
                    }
                },
                Action::Acc => continue
            }
        }
    };
    panic!()
}

fn main() {
    let contents = read_file("input.txt");
    let instructions = contents.trim()
                               .split("\n")
                               .map(|instruction| instruction.split(" ").collect::<Vec<&str>>())
                               .collect::<Vec<Vec<&str>>>();

    let instruction_set = build_instruction_set(instructions);


    let part_1_result = part_1(&instruction_set);
    println!("part 1: {}", part_1_result);

    let mut now = Instant::now();
    let mut part_2_result = part_2_brute(&instruction_set);
    println!("brute time: {:?}", Instant::now().duration_since(now));

    now = Instant::now();
    part_2_result = part_2_not_quite_brute(&instruction_set);
    println!("maybe not brute time: {:?}", Instant::now().duration_since(now));

    println!("part 2: {}", part_2_result);
}
