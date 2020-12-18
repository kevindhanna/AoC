use std::io::Read;
use std::collections::HashMap;

type Layer = Vec<Vec<char>>;
type Cube = Vec<Layer>;
type Point = (i32, i32, i32, i32);
type Hypercube = Vec<Point>;

fn main() {
    let contents = read_file("input.txt");
    let layer = contents.trim()
                       .split("\n")
                       .map(|row| row.chars().collect::<Vec<char>>())
                       .collect::<Layer>();


    part_1_test();
    let part_1_result = part_1(&layer);
    println!("part 1: {}", part_1_result);

    part_2_test();
    let part_2_result = part_2(&layer);
    println!("part 2: {}", part_2_result);
}

fn part_2(z0: &Layer) -> u32 {
    let mut current = Hypercube::new();
    for (x, row) in z0.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == '#' {
                current.push((x as i32, y as i32, 0, 0));
            }
        }
    }
    for i in 0..6 {
        let mut next = Hypercube::new();
        let mut memo: HashMap<Point, u32> = HashMap::new();
        for point in &current {
            let n = number_of_neighbours(&point, &current);
            if n == 2 || n == 3 {
                next.push(point.clone());
            }
            memo.insert(*point, n);
            for dx in 1..=3 {
                let dx = dx as i32 - 2;
                for dy in 1..=3 {
                    let dy = dy as i32 - 2;
                    for dz in 1..=3 {
                        let dz = dz as i32 -2;
                        for dw in 1..=3 {
                            let dw = dw as i32 - 2;
                            let point_offset = (point.0 + dx, point.1 + dy, point.2 + dz, point.3 + dw);
                            if !current.contains(&point_offset) {
                                let n: u32;
                                match memo.get(&point_offset) {
                                    Some(val) => {
                                        n = *val;
                                    },
                                    None => {
                                        n = number_of_neighbours(&point_offset, &current);
                                        memo.insert(point_offset, n);
                                    }
                                }
                                if n == 3 {
                                    next.push(point_offset);
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut next_sorted = Hypercube::new();
        for point in next {
            if next_sorted.contains(&point) {
                continue;
            }
            next_sorted.push(point);
        }
        current = next_sorted;
        println!("at {}. found: {}", i, current.len());
    }

    current.len() as u32
}

fn number_of_neighbours(point: &Point, hypercube: &Hypercube) -> u32 {
    let mut total = 0;
    for dx in 1..=3 {
        let dx = dx as i32 - 2;
        for dy in 1..=3 {
            let dy = dy as i32 - 2;
            for dz in 1..=3 {
                let dz = dz as i32 -2;
                for dw in 1..=3 {
                    let dw = dw as i32 - 2; {
                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                            let point_offset = (point.0 + dx, point.1 + dy, point.2 + dz, point.3 + dw);
                            if hypercube.contains(&point_offset) {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    total
}

fn part_1(z0: &Layer) -> u32 {
    let mut iteration: usize = 0;
    let mut cube = vec![z0.clone()];
    while iteration < 6 {
        cube = expand_cube(cube);
        let mut next = cube.clone();
        for (z, layer) in cube.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let neighbours = find_neighbours(x, y, z, &cube);
                    match cell {
                        '#' => {
                            if neighbours == 3 || neighbours == 4 {
                                // it'll always have itself as a neighbour
                                next[z][y][x] = '#';
                            } else {
                                next[z][y][x] = '.';
                            }
                        },
                        '.' => {
                            if neighbours == 3 {
                                next[z][y][x] = '#';
                            } else {
                                next[z][y][x] = '.';
                            }
                        },
                        _ => panic!("How'd we get here?")
                    }
                }
            }
        }
        cube = next;
        iteration += 1
    }
    count_active(cube)
}

fn count_active(cube: Cube) -> u32 {
    let mut total = 0;
    for layer in 1..cube.len() {
        total += count_layer(&cube[layer]);
    }

    total
}

fn count_layer(layer: &Layer) -> u32 {
    let mut total = 0;
    for row in layer {
        for cell in row {
            if cell == &'#' {
                total += 1;
            }
        }
    }
    total
}

fn expand_cube(mut cube: Cube) -> Cube {
    let len = cube[0][0].len();
    for z_i in 0..cube.len() {
        for x_i in 0..len {
            cube[z_i][x_i].insert(0, '.');
            cube[z_i][x_i].push('.');
        }
        cube[z_i].insert(0, vec!['.'; len + 2]);
        cube[z_i].push(vec!['.'; len + 2]);
    }
    cube.push(vec![vec!['.'; len + 2]; len + 2]);
    cube.insert(0, vec![vec!['.'; len + 2]; len + 2]);
    cube
}

fn find_neighbours(x: usize, y: usize, z: usize, cube: &Cube) ->u32 {
    let mut neighbours = 0;
    let x_min: usize = min(x);
    let x_max: usize = max(x, cube[0].len() - 1);
    let y_min: usize = min(y);
    let y_max: usize = max(y, cube[0].len() - 1);
    let z_min: usize = min(z);
    let z_max: usize = max(z, cube.len() - 1);

    for z in z_min..=z_max {
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                match cube[z][y][x] {
                    '#' => neighbours += 1,
                    _ => {}
                }
            }
        }
    }
    neighbours
}

fn min(i: usize) -> usize {
    match i == 0 {
        true => i,
        false => i - 1
    }
}

fn max(i: usize, len: usize) -> usize {
    match i == len {
        true => i,
        false => i + 1
    }
}

fn part_2_test() {
    let start = vec![
        vec!['.','#', '.'],
        vec!['.','.', '#'],
        vec!['#','#', '#'],
    ];
    assert_eq!(part_2(&start), 848);
}

fn part_1_test() {
    let start = vec![
        vec!['.','#', '.'],
        vec!['.','.', '#'],
        vec!['#','#', '#'],
    ];
    assert_eq!(part_1(&start), 112);
}

fn print_cube(cube: &Cube) {
    for (i, layer) in cube.iter().enumerate() {
        println!("z = {}", i);
        for row in layer {
            for cell in row {
                print!("{}", cell)
            }
            println!("");
        }
        println!("");
    }
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
