const SUB_NUM: u64 = 7;
const DIV: u64 = 20201227;

fn main() {
    let key_1 = 8335663;
    let key_2 = 8614349;

    part_1_test();
    let p1 = part_1(key_1, key_2);
    println!("part 1: {}", p1);
}

fn part_1(key: u64, door: u64) -> u64 {
    let mut lsize = 0;
    let mut pkey = 1;
    let mut k_loop: Option<u64> = None;
    let mut d_loop: Option<u64> = None;
    loop {
        pkey = (pkey * SUB_NUM) % DIV;
        lsize += 1;
        if pkey == key {
            k_loop = Some(lsize);
            break;
        }
        if pkey == door {
            d_loop = Some(lsize);
            break;
        }
    }
    if let Some(d) = d_loop {
        return transform(key, d);
    }
    if let Some(k) = k_loop {
        return transform(door, k);
    }
    panic!("hmmm");
}

fn transform(mut key: u64, lsize: u64) -> u64 {
    let mut e_key = 1;
    for i in 0..lsize {
        e_key = (e_key * key) % DIV;
    }
    e_key
}

fn part_1_test() {
    let key_1 = 5764801;
    let key_2 = 17807724;

    assert_eq!(transform(key_2, 8), 14897079);
    assert_eq!(transform(key_1, 11), 14897079);
    assert_eq!(part_1(key_1, key_2), 14897079);
}
