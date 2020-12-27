fn main() {
    let input: Vec<usize> = "523764819".chars().map(|c| c.to_digit(10).expect("not a digit") as usize).collect();

    part_1_test();
    let part_1_result = part_1(input.clone(), 100);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_2(input.clone(), 10_000_000);
    println!("part 2: {}", part_2_result);
}

fn part_2(mut cups: Vec<usize>, moves: usize) -> usize {
    for i in 10..=1_000_000 {
        cups.push(i);
    }

    let circle = game(cups, moves);
    circle[1] * circle[circle[1]]
}

fn part_1(cups: Vec<usize>, moves: usize) -> String {
    let circle = game(cups, moves);
    build_result(circle)
}

fn game(cups: Vec<usize>, moves: usize) -> Vec<usize> {
    let min = *cups.iter().min().expect("couldn't get min");
    let max = *cups.iter().max().expect("couldn't get max");

    let len = cups.len();
    let mut circle = vec![0; max + 1];
    for i in 0..len {
        circle[cups[i]] = cups[(i + 1) % len];
    }

    let mut first = cups[0];
    for _i in 0..moves {
        calculate_move(&mut circle, first, min, max);
        first = circle[first];
    }
    circle
}

fn build_result(circle: Vec<usize>) -> String {
    let mut result = String::new();
    let mut i = 1;
    while circle[i] != 1 {
        result.push_str(&circle[i].to_string());
        i = circle[i];
    }
    result
}

fn calculate_move(circle: &mut Vec<usize>, first: usize, min: usize, max: usize) {
    let p1 = circle[first];
    let p2 = circle[p1];
    let p3 = circle[p2];
    circle[first] = circle[p3];

    let mut dest = if first > min { first - 1 } else { max };
    while [p1, p2, p3].contains(&dest) || dest < min || dest > max  {
        dest = if dest > min { dest - 1 } else { max };
    }

    let tmp = circle[dest];
    circle[dest] = p1;
    circle[p1] = p2;
    circle[p2] = p3;
    circle[p3] = tmp;
}

fn part_1_test() {
    let input: Vec<usize> = "389125467".chars().map(|c| c.to_digit(10).expect("not a digit") as usize).collect();
    assert_eq!(part_1(input, 10), "92658374".to_owned())
}
