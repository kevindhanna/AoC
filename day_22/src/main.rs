use std::io::Read;
use std::collections::HashSet;

type Deck = Vec<u32>;

fn main() {
    let contents = read_file("input.txt");
    let decks = contents.trim()
                        .split("\n\n")
                        .collect::<Vec<&str>>();

    let player_1 = build_deck(decks[0]);
    let player_2 = build_deck(decks[1]);

    let part_1_result = part_1(&player_1, &player_2);
    println!("part 1: {}", part_1_result);

    part_2_test();
    let part_2_result = part_2(player_1, player_2);
    println!("part 2: {}", part_2_result);
}

fn part_2(p1: Deck, p2: Deck) -> u32 {
    let (p1, p2) = new_game(p1, p2);

    calculate_winner(p1, p2)
}

fn new_game(mut p1: Deck, mut p2: Deck) -> (Deck, Deck) {
    let mut history = HashSet::new();

    while p1.len() > 0 && p2.len() > 0 {
        if !history.insert((p1.clone(), p2.clone())) {
            while p2.len() > 0 {
                p1.insert(0, p2.pop().unwrap());
            }
            break
        }

        let p1c = p1.pop().unwrap();
        let p2c = p2.pop().unwrap();

        let mut p1_win = winner(p1c, p2c);

        if p1.len() as u32 >= p1c && p2.len() as u32 >= p2c {
            let p1_sub = trim(p1.clone(), p1c);
            let p2_sub = trim(p2.clone(), p2c);
            let (p1_new, p2_new) = new_game(p1_sub, p2_sub);

            if p1_new.len() > p2_new.len() {
                p1_win = true;
            } else {
                p1_win = false;
            }

        }

        if p1_win {
            p1.insert(0, p1c);
            p1.insert(0, p2c);
        } else {
            p2.insert(0, p2c);
            p2.insert(0, p1c);
        }
    }

    (p1, p2)
}

fn winner(p1: u32, p2: u32) -> bool {
    p1 > p2
}

fn trim(mut deck: Deck, len: u32) -> Deck {
    while deck.len() > len as usize {
        deck.remove(0);
    }
    deck
}

fn part_1(p1: &Deck, p2: &Deck) -> u32 {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    while p1.len() > 0 && p2.len() > 0 {
        let p1_card = p1.pop().unwrap();
        let p2_card = p2.pop().unwrap();
        if p1_card > p2_card {
            p1.insert(0, p1_card);
            p1.insert(0, p2_card);
        } else {
            p2.insert(0, p2_card);
            p2.insert(0, p1_card);
        }
    }
    calculate_winner(p1, p2)
}

fn calculate_winner(left: Deck, right: Deck) -> u32 {
    let winner: Deck;
    if right.len() == 0 {
        winner = left;
    } else {
        winner = right;
    }

    winner.iter().enumerate().map(|(i, c)| (i + 1) as u32 * c).sum()
}

fn build_deck(deck: &str) -> Deck {
    let mut deck = deck.split("\n").skip(1).collect::<Vec<&str>>();
    deck.reverse();
    deck.iter().map(|c| c.parse::<u32>().unwrap()).collect::<Deck>()
}

fn part_2_test() {
    let p1 = vec![1, 3, 6, 2, 9];
    let p2 = vec![10, 7, 4, 8, 5];
    assert_eq!(part_2(p1, p2), 291);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
