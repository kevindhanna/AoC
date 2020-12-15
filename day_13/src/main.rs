use std::io::Read;

fn main() {
    let contents = read_file("input.txt");
    let parts = contents.split("\n").collect::<Vec<&str>>();

    let arrival = parts[0].parse::<i64>().unwrap();
    let buses = parts[1].trim()
                            .replace("x,", "")
                            .split(",")
                            .map(|bus| bus.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();

    part_1_test();
    let part_1_result = part_1(arrival, buses);

    part_2_test();
    let buses: Vec<Option<i64>> = parts[1].trim()
                                 .split(",")
                                 .map(|bus| {
                                     match bus.parse::<i64>() {
                                         Ok(val) => Some(val),
                                         Err(_) => None
                                     }
                                 })
                                 .collect::<Vec<Option<i64>>>();
    let part_2_result = part_2(&buses);

    println!("part 1: {}", part_1_result);
    println!("part 2: {}", part_2_result);
}

fn part_1(time: i64, mut buses: Vec<i64>) -> i64 {
    buses.sort();
    let mut smallest: (i64, i64) = (buses[buses.len() - 1], time * time); // some arbitrarily large number to avoid using smallest unassigned
    for bus in buses {
        let mut bus_time = bus.clone();
        while bus_time < time {
            bus_time += bus;
        }
        let diff = bus_time - time;
        if smallest.1 > diff {
            smallest.0 = bus.clone();
            smallest.1 = diff;
        }
    }
        smallest.0 * smallest.1
}

fn part_2(buses: &Vec<Option<i64>>) -> i64 {
    let mut time: i64 = 0;

    let mut offset: i64 = 1;
    let mut inc = buses[0].unwrap();

    for bus in &buses[1..] {
        if let Some(bus) = bus {
            while (time + offset) % bus != 0 {
                time += inc;
            }
            inc *= bus;
        }

        offset += 1;
    }

    time
}

fn part_1_test() {
    let parts = vec![
        "939",
        "7,13,x,x,59,x,31,19"
    ];
    let arrival = parts[0].parse::<i64>().unwrap();
    let buses = parts[1].trim()
                            .replace("x,", "")
                            .split(",")
                            .map(|bus| bus.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();
    assert_eq!(part_1(arrival, buses), 295);
}

fn part_2_test() {
    let sets = vec! [
        vec![
            "939",
            "67,7,59,61"
        ],
        vec![
            "939",
            "67,x,7,59,61"
        ],
        vec![
            "939",
            "67,7,x,59,61"
        ],
        vec![
            "939",
            "1789,37,47,1889"
        ],

        vec![
            "939",
            "17,x,13,19"
        ],
        vec![
            "939",
            "7,13,x,x,59,x,31,19"
        ],
    ];

    let mut inputs: Vec<Vec<Option<i64>>> = Vec::new();

    for set in sets {
        inputs.push(set[1].trim()
                    .split(",")
                    .map(|bus| {
                        match bus.parse::<i64>() {
                            Ok(val) => Some(val),
                            Err(_) => None
                        }
                    })
                    .collect::<Vec<Option<i64>>>());

    }
    assert_eq!(part_2(&inputs[0]), 754018);
    assert_eq!(part_2(&inputs[1]), 779210);
    assert_eq!(part_2(&inputs[2]), 1261476);
    assert_eq!(part_2(&inputs[3]), 1202161486);
    assert_eq!(part_2(&inputs[4]), 3417);
    assert_eq!(part_2(&inputs[5]), 1068781);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
