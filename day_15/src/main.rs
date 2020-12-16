use std::collections::HashMap;

fn main() {
    let input = "6,4,12,1,20,0,16".split(",")
                                  .map(|n| n.parse::<usize>().unwrap())
                                  .collect::<Vec<usize>>();

    part_1_test();
    let part_1_result = part_1(&input, 2020);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_1(&input, 30000000);
    println!("part 2: {}", part_2_result);
}

fn part_1(input: &Vec<usize>, num: usize) -> usize {
    let mut history = input.clone();
    let mut memo = HashMap::new();
    for (i, starter) in history[0..history.len() - 1].iter().enumerate() {
        memo.insert(*starter, i);
    }
    let mut turn = history.len() - 1;
    let mut last = history[turn];
    loop {
        if turn == num - 1 {
            break;
        }
        let next: usize;
        match memo.get(&last) {
            Some(i) => next = turn - *i,
            None => next = 0
        }
        memo.insert(last, turn);
        history.push(next);
        last = next;
        turn += 1;
    }
    history[turn]
}

fn part_1_test() {
    let inputs = [
        ("0,3,6", 436),
        ("1,3,2", 1),
        ("2,1,3", 10),
        ("1,2,3", 27),
        ("2,3,1", 78),
        ("3,2,1", 438),
        ("3,1,2", 1836)
    ];
    for input in inputs.iter() {
        let nums = input.0.split(",")
                          .map(|n| n.parse::<usize>().unwrap())
                          .collect::<Vec<usize>>();
        assert_eq!(part_1(&nums, 2020), input.1);
    }
}
