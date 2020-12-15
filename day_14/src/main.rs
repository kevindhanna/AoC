use std::io::Read;
use std::collections::HashMap;

enum Instruction {
    Mask(Vec<char>),
    Mem(u64, u64)
}

fn main() {
    let contents = read_file("input.txt");
    let instructions = contents.trim()
                               .split('\n')
                               .map(|instruction| {
                                   if instruction.contains("mask") {
                                       let mask = instruction.replace("mask = ", "")
                                                             .chars()
                                                             .collect::<Vec<char>>();
                                       return Instruction::Mask(mask);
                                   }
                                   let instruction = instruction.replace("mem[", "")
                                                                .split("] = ")
                                                                .map(|i| i.parse::<u64>().unwrap())
                                                                .into_iter()
                                                                .collect::<Vec<u64>>();
                                   return Instruction::Mem(instruction[0], instruction[1]);
                               })
                               .collect::<Vec<Instruction>>();

    part_1_test();
    let part_1_result = part_1(&instructions);
    println!("part 1: {}", part_1_result);

    part_2_test();
    let part_2_result = part_2(&instructions);
    println!("part 2: {}", part_2_result);
}

fn part_1(instructions: &Vec<Instruction>) -> u64 {
    let mut heap = HashMap::new();

    let mut mask: &Vec<char> = &Vec::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(i, val) => {heap.insert(*i, apply_mask_1(*val, mask));}
        }
    }

    let mut total: u64 = 0;
    for (_key, val) in heap.iter() {
        total += val;
    }
    total
}

fn part_2(instructions: &Vec<Instruction>) -> u64 {
    let mut heap = HashMap::new();

    let mut mask: Vec<char> = Vec::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => mask = m.clone(),
            Instruction::Mem(i, val) => {
                let address = apply_mask_2(u64_to_vec(*i), &mask);
                let addresses = address_variants(address, &mask);
                for address in addresses {
                    heap.insert(address, val);
                }
            }
        }
    }

    let mut total: u64 = 0;
    for (_key, val) in heap.iter() {
        total += **val;
    }
    total
}

fn address_variants(address: Vec<char>, mask: &Vec<char>) -> Vec<u64> {
    if !mask.contains(&'X') {
        return vec![vec_to_u64(address)];
    }
    let mut variants: Vec<u64> = Vec::new();
    for (i, c) in mask.iter().enumerate() {
        match c {
            'X' => {
                let mut v_mask = mask.clone();
                let mut v_address_0 = address.clone();
                let mut v_address_1 = address.clone();
                v_mask[i] = '0';
                v_address_1[i] = '1';
                v_address_0[i] = '0';
                variants.append(&mut address_variants(v_address_1, &v_mask.clone()));
                variants.append(&mut address_variants(v_address_0, &v_mask.clone()));
                break;
            },
            _ => continue
        }
    }
    variants
}

fn apply_mask_2(mut address: Vec<char>, mask: &Vec<char>) -> Vec<char> {
    for (i, c) in mask.iter().enumerate() {
        match c {
            '1' => address[i] = '1',
            _ => continue
        }
    }
    address
}

fn apply_mask_1(val: u64, mask: &Vec<char>) -> u64 {
    let mut val = u64_to_vec(val);

    for (i, c) in mask.iter().enumerate() {
        match c {
            'X' => continue,
            _ => val[i] = *c
        }
    }
    vec_to_u64(val)
}

fn mask_test(mask: &Vec<char>) {
    assert_eq!(apply_mask_1(11, mask), 73);
    assert_eq!(apply_mask_1(101, mask), 101);
    assert_eq!(apply_mask_1( 0, mask), 64);
}

fn vec_to_u64(address: Vec<char>) -> u64 {
    isize::from_str_radix(address.iter().collect::<String>().as_str(), 2).unwrap() as u64
}

fn u64_to_vec(address: u64) -> Vec<char> {
    let mut address = format!("{:b}", address).chars().collect::<Vec<char>>();
    while address.len() < 36 {
        address.insert(0, '0');
    }
    address
}


fn part_1_test() {
    let mask = Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".chars().collect::<Vec<char>>());
    let instructions = vec![
        mask,
        Instruction::Mem(8, 11),
        Instruction::Mem(7, 101),
        Instruction::Mem(8, 0),
    ];
    if let Instruction::Mask(mask) = &instructions[0] {
        mask_test(&mask);
    }

    assert_eq!(part_1(&instructions), 165);
}

fn part_2_test() {
    let instructions = vec![
        Instruction::Mask("000000000000000000000000000000X1001X".chars().collect::<Vec<char>>()),
        Instruction::Mem(42, 100),
        Instruction::Mask("00000000000000000000000000000000X0XX".chars().collect::<Vec<char>>()),
        Instruction::Mem(26, 1),
    ];

    assert_eq!(part_2(&instructions), 208);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
