use std::io::Read;
#[derive(Clone)]

struct Node {
    jolts: u32,
    value: u64
}

fn main() {
    let contents = read_file("input.txt");
    let mut adaptors = contents.trim()
                          .split("\n")
                          .map(|number| number.parse::<u32>().unwrap())
                          .collect::<Vec<u32>>();
    adaptors.sort();
    let part_1_result = part_1(&adaptors);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_2(&adaptors);
    println!("part 2: {}", part_2_result);

}

fn part_1(adaptors: &Vec<u32>) -> u32 {
    let mut ones: u32 = 0 + adaptors[0];
    let mut threes: u32 = 1;
    for i in 0..adaptors.len() - 1 {
        if adaptors[i + 1] - adaptors[i] > 1 {
            ones += 1;
        } else {
            threes += 1;
        }

    }
    ones * threes
}

fn part_2(adaptors: &Vec<u32>) -> u64 {
    let mut adaptors: Vec<Node> = adaptors.iter()
                                          .map(|adaptor| Node{
                                              jolts: *adaptor,
                                              value: 0
                                          })
                                          .collect();
    adaptors.push(Node {
        jolts: adaptors[adaptors.len() - 1].jolts + 3,
        value: 0
    });
    adaptors.reverse();

    let head = Node {
        jolts: 0,
        value: 1
    };

    walk_and_count(head, Vec::new(), adaptors)
}

fn walk_and_count(mut head: Node, mut tail: Vec<Node>, mut adaptors: Vec<Node>) -> u64 {
    for child in &tail {
        if head.jolts - child.jolts < 4 {
            head.value += child.value;
        }
    }
    if adaptors.len() == 0 {
        return head.value;
    }
    tail.push(head);
    head = adaptors.pop().unwrap();
    return walk_and_count(head, tail, adaptors);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
