use std::io::Read;

#[derive(Clone)]
struct Tile {
    id: u64,
    map: Vec<Vec<char>>
}

struct Sides {
    top: String,
    bottom: String,
    left: String,
    right: String
}

struct Join {
    y: i32,
    x: i32
}

impl Tile {
    fn left(&self) -> String {
        self.map.iter().map(|line| line[0]).collect::<String>()
    }
    fn right(&self) -> String {
        self.map.iter().map(|line| line[line.len()-1]).collect::<String>()
    }
    fn top(&self) -> String {
        self.map[0].iter().collect::<String>()
    }
    fn bottom(&self) -> String {
        self.map[self.map.len() - 1].iter().collect::<String>()
    }
}

fn main() {
    let contents = read_file("input.txt");
    let tiles = build_tiles(contents);

    // tiles_test();
    // part_1_tests();
    // let part_1_result = part_1(tiles);
    // println!("part 1: {}", part_1_result.0);

    part_2_test();
    // let part_2_result = part_2(part_1_result.1);
    // println!("part 2: {}", part_2_result);
}

fn part_2(grid: Vec<Vec<Tile>>) -> u32 {
    println!("{}", grid[0][1].id);
    _print_tile(&grid[0][1]);
    let grid = make_whole_and_trim(grid);
    // for line in grid {
    //     println!("{}", line.iter().collect::<String>());
    // }
    1
}

fn make_whole_and_trim(grid: Vec<Vec<Tile>>) -> Vec<Vec<char>> {
    let mut whole_grid: Vec<Vec<char>> = Vec::new();
    let mut buffer: Vec<Vec<String>> = vec![vec!["".to_owned(); grid[0][0].map.len()]; grid[0].len()];
    for k in 0..grid[0][0].map.len() {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                buffer[i][k].push_str(grid[i][j].map[k].iter().collect::<String>().as_str());
            }
        }
    }
    for chunk in buffer {
        for line in chunk {
            whole_grid.push(line.chars()
                            // .collect::<Vec<char>>()[1..line.len() - 1]
                            // .iter()
                            // .map(|c| *c)
                            .collect::<Vec<char>>());
        }
    }
    // whole_grid.pop();
    // whole_grid.remove(whole_grid.len() - 1);
    whole_grid
}

fn part_1(mut tiles: Vec<Tile>) -> (u64, Vec<Vec<Tile>>) {
    let len = (tiles.len() as f64).sqrt() as usize;

    let mut grid: Vec<Vec<Tile>> = vec![vec![empty_grid(tiles[0].map.len()); len * 3]; len * 3];
    let mut tile_coords: Vec<(usize, usize)> = Vec::new();

    let mut first = tiles.pop().unwrap();

    first = rotate(first);
    first = flip(first);
    first = rotate(first);
    first = flip(first);

    grid[len][len] = first;
    tile_coords.push((len, len));

    while tiles.len() > 0 {
        let t = tiles.pop().unwrap();
        let mut matched = false;
        for i in 0..tile_coords.len() {
            let tile = &grid[tile_coords[i].0][tile_coords[i].1];
            if let Some((t, join)) = find_join(tile.clone(), t.clone()) {
                let x = (tile_coords[i].1 as i32 + join.x) as usize;
                let y = (tile_coords[i].0 as i32 + join.y) as usize;
                if !tile_coords.contains(&(y, x)) {
                    grid[y][x] = t;
                    tile_coords.push((y, x));
                    matched = true;
                    break;
                }
            }
        }
        if !matched {
            tiles.insert(0, t.clone());
        }
    }
    let grid = clear_whitespace(grid, len);
    _print_grid(&grid);

    (grid[0][0].id * grid[0][len - 1].id * grid[len - 1][0].id * grid[len - 1][len - 1].id, grid)
}

fn clear_whitespace(mut grid: Vec<Vec<Tile>>, len: usize) -> Vec<Vec<Tile>> {
    let mut new_grid: Vec<Vec<Tile>> = Vec::new();
    while grid.len() > 0 {
        let mut line = grid.pop().unwrap();
        loop {
            if line.len() == len {
                if line[0].map[0][0] != ' ' {
                    new_grid.push(line);
                }
                break
            }
            for i in 0..line.len() {
                if line[i].map[0][0] == ' ' {
                    line.remove(i);
                    break;
                }
            }
        }
    }

    new_grid
}

fn find_join(tile1: Tile, mut tile2: Tile) -> Option<(Tile, Join)> {
    let sides = Sides { top: tile1.top(), bottom: tile1.bottom(), left: tile1.left(), right: tile1.right() };
    for _i in 1..=2 {
        for _j in 1..=4 {
            if tile2.bottom() == sides.top {
                let join = Join { x: 0, y: 1 };
                return Some((tile2, join))
            }
            if tile2.top() == sides.bottom {
                let join = Join{ x: 0, y: -1 };
                return Some((tile2, join))
            }
            if tile2.right() == sides.left {
                let join = Join{ x: -1, y: 0 };
                return Some((tile2, join))
            }
            if tile2.left() == sides.right {
                let join = Join{ x: 1, y: 0 };
                return Some((tile2, join))
            }
            tile2 = rotate(tile2);
        }
        tile2 = flip(tile2);
    }
    None
}

fn build_tiles(contents: String) -> Vec<Tile> {
    contents.split("\n\n")
            .map(|t| {
                let tile = t.trim().split("\n").collect::<Vec<&str>>();
                let id = tile[0].replace("Tile ", "")
                                .replace(":", "")
                                .parse::<u64>().unwrap();
                return Tile {
                    id,
                    map: tile[1..]
                        .iter()
                        .map(|line| line.chars().collect::<Vec<char>>())
                        .collect::<Vec<Vec<char>>>()
                }
            })
            .collect::<Vec<Tile>>()
}

fn rotate(tile: Tile) -> Tile {
    let mut map = tile.map.clone();
    let n = map.len();
    for x in 0..n {
        for y in x..n {
            let xy = map[x][y];
            map[x][y] = map[y][x];
            map[y][x] = xy;
        }
    }

    for x in 0..n {
        for y in 0..n / 2 {
            let xy = map[x][y];
            map[x][y] = map[x][n - y - 1];
            map[x][n - y - 1] = xy;
        }
    }
    Tile {
        id: tile.id,
        map
    }
}

fn flip(tile: Tile) -> Tile {
    Tile {
        id: tile.id,
        map: tile.map.iter().map(|line| {
            return line.iter().rev().map(|c| *c).collect::<Vec<char>>();
        }).collect::<Vec<Vec<char>>>()
    }
}

fn empty_grid(len: usize) -> Tile {
    let line = vec![' '; len];
    Tile {
        id: 0,
        map: vec![line.clone(); len]
    }
}

fn _print_tile(tile: &Tile) {
    for line in &tile.map {
        println!("{}", line.iter().collect::<String>());
    }
    println!("");
}

fn _print_grid(grid: &Vec<Vec<Tile>>) {
    let mut buffer: Vec<Vec<String>> = vec![vec!["".to_owned(); grid[0][0].map.len()]; grid[0].len()];
    for k in 0..grid[0][0].map.len() {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                buffer[i][k].push_str(grid[i][j].map[k].iter().collect::<String>().as_str());
                buffer[i][k].push_str(" ");
            }
        }
    }
    for set in buffer {
        for line in set {
            println!("{}", line);
        }
        println!("");
    }
}

fn part_2_test() {
    let contents = read_file("test_input.txt");
    let tiles = build_tiles(contents);
    let grid = part_1(tiles).1;

    assert_eq!(part_2(grid), 273);
}
fn part_1_tests() {
    let contents = read_file("test_input.txt");
    let tiles = build_tiles(contents);
    tiles_test();

    assert_eq!(part_1(tiles).0, 20899048083289);
}

fn tiles_test() {
    let map ="#.#.#####.\n\
              .#..######\n\
              ..#.......\n\
              ######....\n\
              ####.#..#.\n\
              .#...#.##.\n\
              #.#####.##\n\
              ..#.###...\n\
              ..#.......\n\
              ..#.###...".split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let tile = Tile {
        id: 1,
        map
    };
    assert_eq!(tile.left(), "#..##.#...");
    assert_eq!(tile.right(), ".#....#...");
    assert_eq!(tile.top(), "#.#.#####.");
    assert_eq!(tile.bottom(), "..#.###...");

    let tile = rotate(tile);

    assert_eq!(tile.left(), "..#.###...");
    assert_eq!(tile.right(), "#.#.#####.");
    assert_eq!(tile.top(), "...#.##..#");
    assert_eq!(tile.bottom(), "...#....#.");

    let tile = flip(tile);

    assert_eq!(tile.top(), "#..##.#...");
    assert_eq!(tile.bottom(), ".#....#...");
    assert_eq!(tile.right(), "..#.###...");
    assert_eq!(tile.left(), "#.#.#####.");
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
