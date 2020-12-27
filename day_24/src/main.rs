use std::collections::HashSet;
use std::io::Read;
use regex::Regex;

type Tile = (i32, i32, i32);

const DIRECTIONS: [(i32, i32, i32); 6] = [
    (1, -1, 0),
    (-1, 1, 0),
    (0, 1, -1),
    (1, 0, -1),
    (0, -1, 1),
    (-1, 0, 1),
];

fn main() {
    let input = read_file("input.txt");
    let instructions = build_instructions(input.trim());

    part_1_test();
    let (p1, floor) = part_1(instructions);
    println!("part 1: {}", p1);

    part_2_test();
    let p2 = part_2(floor, 100);
    println!("part 2: {}", p2);
}

fn part_2(mut floor: HashSet<Tile>, days: usize) -> usize {
    for _ in 0..days {
        floor = iterate(floor);
    }

    floor.len()
}

fn iterate(floor: HashSet<Tile>) -> HashSet<Tile> {
    let mut iterant = HashSet::new();

    for tile in &floor {
        let ns = count_neighbours(tile, &floor);
        if ns == 1 || ns == 2 {
            iterant.insert(tile.clone());
        }
        for d in DIRECTIONS.iter() {
            let neighbour = (tile.0 + d.0, tile.1 + d.1, tile.2 + d.2);
            if iterant.contains(&neighbour) { continue };
            let ns = count_neighbours(&neighbour, &floor);
            if ns == 2 {
                iterant.insert(neighbour);
            }
        }
    }
    iterant
}

fn count_neighbours(tile: &Tile, floor: &HashSet<Tile>) -> u32 {
    let mut count = 0;
    let (x, y, z) = tile;
    for d in DIRECTIONS.iter() {
        if floor.contains(&(x + d.0, y + d.1, z + d.2)) {
            count += 1;
        }
    }
    count
}

fn part_1(instructions: Vec<Vec<usize>>) -> (usize, HashSet<Tile>) {
    let mut floor: HashSet<Tile> = HashSet::new();
    for instruction in instructions {
        let mut current: Tile = (0, 0, 0);
        for i in instruction {
            let (mut x, mut y, mut z) = current;
            let change = DIRECTIONS[i];
            x += change.0;
            y += change.1;
            z += change.2;
            current = (x, y, z);
        }
        match floor.contains(&current) {
            true => floor.remove(&current),
            false => floor.insert(current)
        };
    }

    (floor.len(), floor)
}

fn build_instructions(input: &str) -> Vec<Vec<usize>> {
    let regex = "(se)|(nw)|(ne)|(sw)|(e)|(w)";
    let re = Regex::new(regex).expect("Failed to create regex");
    input.split("\n")
         .map(|instruction| {
             re.find_iter(instruction)
               .map(|i| match i.as_str() {
                   "e" => 0,
                   "w" => 1,
                   "nw" => 2,
                   "ne" => 3,
                   "se" => 4,
                   "sw" => 5,
                   _ => panic!()
               })
               .collect::<Vec<usize>>()
         })
         .collect::<Vec<Vec<usize>>>()
}

fn part_2_test() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw\n\
                 neeenesenwnwwswnenewnwwsewnenwseswesw\n\
                 seswneswswsenwwnwse\n\
                 nwnwneseeswswnenewneswwnewseswneseene\n\
                 swweswneswnenwsewnwneneseenw\n\
                 eesenwseswswnenwswnwnwsewwnwsene\n\
                 sewnenenenesenwsewnenwwwse\n\
                 wenwwweseeeweswwwnwwe\n\
                 wsweesenenewnwwnwsenewsenwwsesesenwne\n\
                 neeswseenwwswnwswswnw\n\
                 nenwswwsewswnenenewsenwsenwnesesenew\n\
                 enewnwewneswsewnwswenweswnenwsenwsw\n\
                 sweneswneswneneenwnewenewwneswswnese\n\
                 swwesenesewenwneswnwwneseswwne\n\
                 enesenwswwswneneswsenwnewswseenwsese\n\
                 wnwnesenesenenwwnenwsewesewsesesew\n\
                 nenewswnwewswnenesenwnesewesw\n\
                 eneswnwswnwsenenwnwnwwseeswneewsenese\n\
                 neswnwewnwnwseenwseesewsenwsweewe\n\
                 wseweeenwnesenwwwswnew";

    let instructions = build_instructions(input);
    let (p1, floor) = part_1(instructions);
    assert_eq!(part_2(floor, 100), 2208);
}

fn part_1_test() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw\n\
                 neeenesenwnwwswnenewnwwsewnenwseswesw\n\
                 seswneswswsenwwnwse\n\
                 nwnwneseeswswnenewneswwnewseswneseene\n\
                 swweswneswnenwsewnwneneseenw\n\
                 eesenwseswswnenwswnwnwsewwnwsene\n\
                 sewnenenenesenwsewnenwwwse\n\
                 wenwwweseeeweswwwnwwe\n\
                 wsweesenenewnwwnwsenewsenwwsesesenwne\n\
                 neeswseenwwswnwswswnw\n\
                 nenwswwsewswnenenewsenwsenwnesesenew\n\
                 enewnwewneswsewnwswenweswnenwsenwsw\n\
                 sweneswneswneneenwnewenewwneswswnese\n\
                 swwesenesewenwneswnwwneseswwne\n\
                 enesenwswwswneneswsenwnewswseenwsese\n\
                 wnwnesenesenenwwnenwsewesewsesesew\n\
                 nenewswnwewswnenesenwnesewesw\n\
                 eneswnwswnwsenenwnwnwwseeswneewsenese\n\
                 neswnwewnwnwseenwseesewsenwsweewe\n\
                 wseweeenwnesenwwwswnew";

    let instructions = build_instructions(input);
    let (p1, _) = part_1(instructions);
    assert_eq!(p1, 10);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
