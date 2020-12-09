use std::io::Read;

fn main() {
    let contents = read_file("input.txt");
    let numbers = contents.trim()
                          .split("\n")
                          .map(|number| number.parse::<u64>().unwrap())
                          .collect::<Vec<u64>>();

    part_1_test();
    part_2_test();
    let part_1_result = part_1(&numbers, 25 as usize);
    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&numbers, 25 as usize);
    println!("Part 2: {}", part_2_result);
}

fn part_1(nums: &Vec<u64>, size: usize) -> u64 {
    nums[find_outlier_index(nums, size)]
}

fn part_2(nums: &Vec<u64>, size: usize) -> u64 {
    let c = find_outlier_index(nums, size);
    let target = nums[c];
    let mut start = c - 2 as usize;
    let mut end = c - 1;
    let mut sum: u64 = 0;
    while sum != target {
        sum = sum_range(nums[start..=end].to_vec());
        if sum > target {
            end -= 1;
        }
        start -= 1;
    }

    sum_biggest_and_smallest(nums[start..=end].to_vec())
}

fn find_outlier_index(nums: &Vec<u64>, size: usize) -> usize {
        let mut c = size;
    let mut i2 = c - 1;
    loop {
        let i1 = c - size;
        match can_sum(c, i1, i2, nums) {
            Ok(_) => {
                i2 = c;
                c += 1;
            },
            Err(_) => {
                i2 -= 1;
                if i2 == i1 {
                    return c
                }
            }
        }
    }
}

fn sum_biggest_and_smallest(mut nums: Vec<u64>) -> u64 {
    nums.sort();
    nums[0] + nums[nums.len() - 1]
}

fn sum_range(nums: Vec<u64>) -> u64 {
    let mut sum = 0;
    for x in nums {
        sum += x;
    }
    sum
}

fn can_sum(candidate_i: usize, i1: usize, i2: usize, nums: &Vec<u64>) -> Result<(), ()> {
    if i1 == i2 {
        return Err(())
    }
    if nums[i1] + nums[i2] == nums[candidate_i] {
        return Ok(())
    }
    can_sum(candidate_i, i1 + 1, i2, nums)
}

fn part_1_test() {
    let nums: Vec<u64> = vec![
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        49,
        100
    ];
    assert_eq!(part_1(&nums, 25 as usize), 100);
}

fn part_2_test() {
    let nums: Vec<u64> = vec![
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576,
    ];
    assert_eq!(part_2(&nums, 5), 62);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
