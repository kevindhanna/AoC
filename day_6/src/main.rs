use std::io::Read;

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn part_1(groups: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for group in groups {
        let answers: String = group.trim().replace("\n", "");
        let mut yes_questions: Vec<char> = Vec::new();
        for answer in answers.chars() {
            if !yes_questions.contains(&answer) {
                yes_questions.push(answer);
            }
        }
        sum += yes_questions.len() as u32;
    }

    sum
}

fn part_2(groups: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for group in groups {
        let mut people: Vec<&str> = group.trim().split("\n").collect();
        let p = people.pop().unwrap();
        let mut answers_in_all = p.len();
        for answer in p.chars() {
            for person in &people {
                if !person.contains(answer) {
                    answers_in_all -= 1;
                    break;
                }
            }
        }
        sum += answers_in_all as u32;
    }

    sum
}

fn main() {
    let contents = read_file("input.txt");
    let groups: Vec<&str> = contents.split("\n\n").collect();

    let part_1_result = part_1(&groups);
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&groups);
    println!("Part 2: {}", part_2_result);
}
