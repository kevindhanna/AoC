use std::io::Read;
use std::str;

struct Slope {
    right: usize,
    down: usize,
}

struct Position {
    x: usize,
    y: usize,
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn is_tree(map: &Vec<Vec<char>>, pos: &Position) -> u32 {
    if map[pos.y][pos.x] == '#' {
        return 1
    }
    return 0;
}

fn extend_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for y in map {
        let mut new_y = y.clone();
        new_y.append(&mut y.clone());
        new_map.push(new_y);
    }
    new_map
}

fn part_1(mut map: Vec<Vec<char>>, slope: Slope) -> u32 {
    let mut pos = Position { x: 0, y: 0 };
    let mut count: u32 = is_tree(&map, &pos);
    let last = map.len();
    while pos.y < last {
        if pos.x >= map[0].len() {
            map = extend_map(&map);
        }
        count += is_tree(&map, &pos);
        pos.y += slope.down;
        pos.x += slope.right;
    }
    count
}

fn part_2(map: Vec<Vec<char>>, slopes: Vec<Slope>) -> u32 {
    let mut count: u32 = 1;
    for slope in slopes {
        count *= part_1(map.clone(), slope);
    }
    count
}

fn is_tree_tests() {
    let mut map: Vec<Vec<char>> = Vec::new();
    map.push(['.','.','.'].to_vec());
    map.push(['#','#','#'].to_vec());
    let pos = Position { x: 0, y: 0 };
    assert_eq!(is_tree(&map, &pos), 0);

    let pos = Position { x: 1, y: 1 };
    assert_eq!(is_tree(&map, &pos), 1);
}

fn extend_map_tests() {
    let mut map: Vec<Vec<char>> = Vec::new();
    map.push(['.','.','.'].to_vec());
    map.push(['#','#','#'].to_vec());
    extend_map(&map);
    assert_eq!(map[0].len(), 6);
    assert_eq!(map[1].len(), 6);
}

fn part_1_tests() {
    let mut map: Vec<Vec<char>> = Vec::new();
    map.push(['.','.','.'].to_vec());
    map.push(['#','#','#'].to_vec());

    assert_eq!(part_1(map, Slope{right: 3, down: 1}), 1);
}

fn main() {
    let contents = read_file("input.txt");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        map.push(line.chars().collect());
    }

    let mut slopes: Vec<Slope> = Vec::new();
    slopes.push(Slope{right: 1, down: 1});
    slopes.push(Slope{right: 3, down: 1});
    slopes.push(Slope{right: 5, down: 1});
    slopes.push(Slope{right: 7, down: 1});
    slopes.push(Slope{right: 1, down: 2});

    let part_1_result = part_1(map.clone(), Slope{right: 3, down: 1});
    let part_2_result = part_2(map.clone(), slopes);
    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);
}
