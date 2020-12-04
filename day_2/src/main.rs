use std::io::Read;
use std::str;

struct Policy {
    min: u32,
    max: u32,
    letter: char,
}

fn parse_policy(string: String) -> Policy {
    let policy: String = string.replace("-", " ");
    let policy: Vec<&str> = policy.split(" ").collect();
    let min: u32 = policy[0].parse::<u32>().unwrap();
    let max: u32 = policy[1].parse::<u32>().unwrap();
    let letter: char = policy[2].parse::<char>().unwrap();

    Policy {
        min,
        max,
        letter
    }
}

fn count_instance_of_letter(letter: char, string: String) -> u32 {
    let mut count = 0;
    for l in string.chars() {
        if l == letter {
            count += 1;
        }
    }

    count
}

fn is_valid_password(password: &str, policy: Policy) -> bool {
    let mut first: bool = false;
    let mut last: bool = false;
    let min: usize = (policy.min - 1) as usize;
    let max: usize = (policy.max - 1) as usize;
    for (i, letter) in password.chars().enumerate() {
        if i == min && letter == policy.letter {
            first = true;
        }

        if i == max && letter == policy.letter {
            last = true;
        }
    }

    (first || last) && !(first && last)
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn part_1(entries: &Vec<&str>) -> u32 {
    let mut valid_passwords: u32 = 0;
    for entry in entries {
        let entry: Vec<&str> = entry.split(": ").collect();
        let password: &str = entry[1];
        let policy: Policy = parse_policy(entry[0].into());
        let count: u32 = count_instance_of_letter(policy.letter, password.into());
        if count >= policy.min && count <= policy.max {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn part_2(entries: &Vec<&str>) -> u32 {
    let mut valid_passwords: u32 = 0;
    for entry in entries {
        let entry: Vec<&str> = entry.split(": ").collect();
        let password: &str = entry[1];
        let policy: Policy = parse_policy(entry[0].into());

        if is_valid_password(password, policy) {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn valid_password_tests() {
    let password1: &str = "abcdefg";
    let password2: &str = "aaaaaaa";
    let password3: &str = "aaaabbb";
    assert!(is_valid_password(password1, Policy {
        min: 1,
        max: 3,
        letter: "a".parse::<char>().unwrap()
    }));

    assert_eq!(is_valid_password(password1, Policy {
        min: 2,
        max: 3,
        letter: "a".parse::<char>().unwrap()
    }), false);

    assert_eq!(is_valid_password(password2, Policy {
        min: 1,
        max: 3,
        letter: "a".parse::<char>().unwrap()
    }), false);

    assert!(is_valid_password(password3, Policy {
        min: 2,
        max: 5,
        letter: "a".parse::<char>().unwrap()
    }));

    assert_eq!(is_valid_password(password3, Policy {
        min: 1,
        max: 2,
        letter: "a".parse::<char>().unwrap()
    }), false);
}

fn part_2_tests() {
    let mut vec = Vec::new();
    vec.push("1-3 a: abcde");
    vec.push("1-3 b: cdefg");
    vec.push("2-9 c: cccccccccc");
    assert_eq!(part_2(&vec), 1);
}

fn main() {
    let contents = read_file("input.txt");
    let passwords: Vec<&str> = contents.lines().collect();

    let part_1_result = part_1(&passwords);
    let part_2_result = part_2(&passwords);

    println!("part 1: {}", part_1_result);
    println!("part 2: {}", part_2_result);
}
