use std::io::Read;

fn main() {
    let mut contents = read_file("input.txt");
    contents = contents.replace("(", "( ");
    contents = contents.replace(")", " )");
    let expressions = contents.trim()
                              .split('\n')
                              .map(|line| line.split(" ").map(|str| str.into()).collect::<Vec<String>>())
                              .collect::<Vec<Vec<String>>>();

    part_1_test();
    let part_1_result = part_1(&expressions);
    println!("part 1: {}", part_1_result);

    part_2_test();
    let part_2_result = part_2(&expressions);
    println!("part 2: {}", part_2_result);
}

fn part_2(expressions: &Vec<Vec<String>>) -> i64 {
    let mut total: i64 = 0;
    for expression in expressions {
        let result = calculate_section_2(expression.clone());
        total += result;
    }
    total
}

fn calculate_section_2(mut expression: Vec<String>) -> i64 {
    if expression.contains(&"(".to_owned()) {
        expression = calculate_brackets_2(expression);
    }
    if expression.contains(&"+".to_owned()) {
        expression = calculate_addition(expression);
    }
    let mut stack: Vec<String> = Vec::new();
    expression.reverse();
    while expression.len() > 0 {
        let top = expression.pop().unwrap();
        match top.as_str() {
            "*" => {
                let prev = stack.pop().unwrap().parse::<i64>().unwrap();
                let next = expression.pop().unwrap().parse::<i64>().unwrap();
                stack.push((prev * next).to_string());
            },
            _ => {
                stack.push(top);
            }
        }
    }
    stack[0].parse::<i64>().unwrap()
}

fn calculate_brackets_2(mut expression: Vec<String>) -> Vec<String> {
    while expression.contains(&"(".to_owned()) {
        let mut i: usize = 0;
        while expression[i] != ")" && i < expression.len() - 1 {
            i += 1;
        }
        let mut j = i;
        while expression[j] != "(" {
            j -= 1;
        }
        let sub = calculate_section_2(expression[j + 1..=i - 1].to_vec());
        for _c in j..=i {
            // as we remove the index of the next char becomes j
            expression.remove(j);
        }
        expression.insert(j, sub.to_string());
    }
    expression
}


fn calculate_addition(mut expression: Vec<String>) -> Vec<String> {
    while expression.contains(&"+".to_owned()) {
        let mut i: usize = 0;
        while expression[i] != "+" && i < expression.len() - 1 {
            i += 1;
        }
        let sub = calculate_section_1(expression[i - 1..=i + 1].to_vec());
        for _c in 0..3 {
            expression.remove(i - 1);
        }
        expression.insert(i - 1, sub.to_string());
    }
    expression
}

fn part_1(expressions: &Vec<Vec<String>>) -> i64 {
    let mut total: i64 = 0;
    for expression in expressions {
        let result = calculate_section_1(expression.clone());
        total += result;
    }
    total
}

fn calculate_section_1(mut expression: Vec<String>) -> i64 {
    if expression.contains(&"(".to_owned()) {
        expression = calculate_brackets_1(expression);
    }
    let mut stack: Vec<String> = Vec::new();
    expression.reverse();
    while expression.len() > 0 {
        let top = expression.pop().unwrap();
        match top.as_str() {
            "+" => {
                let prev = stack.pop().unwrap().parse::<i64>().unwrap();
                let next = expression.pop().unwrap().parse::<i64>().unwrap();
                stack.push((prev + next).to_string());
            },
            "*" => {
                let prev = stack.pop().unwrap();
                let prev = prev.parse::<i64>().unwrap();
                let next = expression.pop().unwrap().parse::<i64>().unwrap();
                stack.push((prev * next).to_string());
            },
            _ => {
                stack.push(top);
            }
        }
    }
    stack[0].parse::<i64>().unwrap()
}

fn calculate_brackets_1(mut expression: Vec<String>) -> Vec<String> {
    while expression.contains(&"(".to_owned()) {
        let mut i: usize = 0;
        while expression[i] != ")" && i < expression.len() - 1 {
            i += 1;
        }
        let mut j = i;
        while expression[j] != "(" {
            j -= 1;
        }
        let sub = calculate_section_1(expression[j + 1..=i - 1].to_vec());
        for _c in j..=i {
            // as we remove the index of the next char becomes j
            expression.remove(j);
        }
        expression.insert(j, sub.to_string());
    }
    expression
}

fn part_1_test() {
    let expressions = vec!["1 + 2 * 3 + 4 * 5 + 6".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_1(&expressions), 71);
    let expressions = vec!["2 * 3 + ( 4 * 5 )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_1(&expressions), 26);
    let expressions = vec!["1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_1(&expressions), 51);
    let expressions = vec!["5 + ( 8 * 3 + 9 + 3 * 4 * 3 )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_1(&expressions), 437);
    let expressions = vec!["5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_1(&expressions), 12240);
    let expressions = vec!["( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_1(&expressions), 13632);
}

fn part_2_test() {
    let expressions = vec!["1 + 2 * 3 + 4 * 5 + 6".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_2(&expressions), 231);
    let expressions = vec!["2 * 3 + ( 4 * 5 )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_2(&expressions), 46);
    let expressions = vec!["1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_2(&expressions), 51);
    let expressions = vec!["5 + ( 8 * 3 + 9 + 3 * 4 * 3 )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_2(&expressions), 1445);
    let expressions = vec!["5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) )".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_2(&expressions), 669060);
    let expressions = vec!["( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2".split(" ").map(|s| s.into()).collect::<Vec<String>>()];
    assert_eq!(part_2(&expressions), 23340);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
