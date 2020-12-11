use std::io::Read;

#[derive(PartialEq,Clone)]
enum Chair {
    Empty,
    Occupied,
    Floor
}

type Row = Vec<Chair>;
type FloorPlan = Vec<Row>;

const PART_1: u32 = 3;
const PART_2: u32 = 4;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1), //up
    (1, 1), // up right
    (1, 0), // right
    (1, -1), // down right
    (0, -1), // down
    (-1, -1), // down left
    (-1, 0), // left
    (-1, 1), // left up
];

fn main() {
    let contents = read_file("input.txt");
    let floor_plan:FloorPlan = contents.trim()
                                       .split("\n")
                                       .map(|row| build_row(row))
                                       .collect();

    part_2_test();
    let part_1_result = part_1(&floor_plan);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_2(&floor_plan);
    println!("part 2: {}", part_2_result);
}

fn part_1(floor_plan: &FloorPlan) -> u32 {
    let floor_plan = fill(&floor_plan, 3);
    count_occupied(floor_plan)
}

fn part_2(floor_plan: &FloorPlan) -> u32 {
    let floor_plan = fill(&floor_plan, 4);
    count_occupied(floor_plan)
}

fn part_2_test() {
    let contents = read_file("test_input.txt");
    let floor_plan:FloorPlan = contents.trim()
                                       .split("\n")
                                       .map(|row| build_row(row))
                                       .collect();

    let result = part_2(&floor_plan);
    assert_eq!(result, 26);
}

fn count_occupied(floor_plan: FloorPlan) -> u32 {
    let mut occupied: u32 = 0;
    for row in floor_plan {
        for chair in row {
            occupied += maybe_add(&chair);
        }
    }
    occupied
}

fn fill(floor_plan: &FloorPlan, fussiness: u32) -> FloorPlan {
    let filled = iterate(&floor_plan, fussiness);
    if is_equal_plan(&filled, &floor_plan) {
        return filled;
    }
    fill(&filled, fussiness)
}

fn iterate(floor_plan: &FloorPlan, fussiness: u32) -> FloorPlan {
    let mut iterant: FloorPlan = Vec::new();
    for (row_i, row) in floor_plan.iter().enumerate() {
        iterant.push(Vec::new());
        for (col_i, _chair) in row.iter().enumerate() {
            iterant[row_i].push(calculate_chair(row_i, col_i, fussiness, floor_plan));
        }
    }
    iterant
}

fn calculate_chair(row: usize, col: usize, fussiness: u32, plan: &FloorPlan) -> Chair {
    let neighbours: u32;
    match fussiness {
        PART_1 => neighbours = find_neighbours(row, col, plan),
        PART_2 => neighbours = find_neighbours_los(row, col, plan),
        _ => panic!("wah")
    }
    match plan[row][col] {
        Chair::Floor => return Chair::Floor,
        _ => {
            if neighbours > fussiness {
                return Chair::Empty;
            }
            if neighbours == 0 {
                return Chair::Occupied;
            }
            plan[row][col].clone()
        }
    }
}

// part 1
fn find_neighbours(row: usize, col: usize, plan: &FloorPlan) -> u32 {
    let mut neighbours = 0;
    if row > 0 {
        neighbours += find_row_neighbours(col, &plan[row - 1], true);
    }
    neighbours += find_row_neighbours(col, &plan[row], false);
    if row < plan.len() - 1 {
        neighbours += find_row_neighbours(col, &plan[row + 1], true);
    }
    neighbours
}

fn find_row_neighbours(index: usize, row: &Vec<Chair>, inc_zero: bool) -> u32 {
    let mut count: u32 = 0;
    if index > 0 {
        count += maybe_add(&row[index - 1]);
    }
    if inc_zero {
        count += maybe_add(&row[index]);
    }
    if index < row.len() - 1 {
        count += maybe_add(&row[index + 1]);
    }
    count
}

fn maybe_add(chair: &Chair) -> u32 {
    match chair {
        Chair::Occupied => 1,
        _ => 0
    }
}

// part 2
fn find_neighbours_los(row: usize, col: usize, plan: &FloorPlan) -> u32 {
    let mut neighbours = 0;
    for direction in DIRECTIONS.iter() {
        neighbours += walk_to_edge(row, col, direction, plan)
    }
    neighbours
}

fn walk_to_edge(mut row: usize, mut col: usize, direction: &(i32, i32), plan: &FloorPlan) -> u32 {
    let (row_mod, col_mod) = direction;

    match mod_index(row, row_mod, plan.len() - 1) {
        Ok(val) => row = val,
        Err(_) => return 0,
    }
    match mod_index(col, col_mod, plan[row].len() - 1) {
        Ok(val) => col = val,
        Err(_) => return 0,
    }

    match plan[row][col] {
        Chair::Occupied => return 1,
        Chair::Empty => return 0,
        Chair::Floor => {}
    }

    walk_to_edge(row, col, direction, plan)
}

fn mod_index(i: usize, val: &i32, end: usize) -> Result<usize, ()> {
    let mask: i32 = 1 + val;
    match mask {
        1 => Ok(i),
        0 => {
            if i == 0 {
                return Err(());
            }
            Ok(i - 1)
        },
        2 => {
            if i == end {
                return Err(());
            }
            Ok(i + 1)
        },
        _ => Err(())
    }
}

fn is_equal_plan(left: &FloorPlan, right: &FloorPlan) -> bool {
    for (row_i, row) in left.iter().enumerate() {
        if !is_equal_row(row, &right[row_i]) {
            return false;
        }
    }
    true
}

fn is_equal_row(left: &Row, right: &Row) -> bool {
    for (col_i, chair) in left.iter().enumerate() {
        if *chair != right[col_i] {
            return false;
        }
    }
    true
}

fn build_row(row: &str) -> Row {
    row.chars()
       .map(|c| {
           match c {
               'L' => Chair::Empty,
               '.' => Chair::Floor,
               _ => panic!()
           }
       })
       .collect()
}

fn _print(iterant: &FloorPlan) {
    println!("{}", iterant
             .iter()
             .map(|row| {
                 let mut string = row.iter()
                                 .map(|c| match c {
                                     Chair::Empty => "[L]",
                                     Chair::Occupied => "[#]",
                                     Chair::Floor => "[.]"
                                 })
                                 .collect::<String>();
                 string.push('\n');
                 return string;
             })
             .collect::<String>());
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
