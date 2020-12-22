use std::io::Read;
use std::time::Instant;

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
    let now = Instant::now();
    let part_2_result = part_2(player_1, player_2);
    println!("time: {:?}", Instant::now().duration_since(now));
    println!("part 2: {}", part_2_result);
}

fn part_2(p1: Deck, p2: Deck) -> u32 {
    let (p1, p2) = new_game(p1, p2);

    calculate_winner(p1, p2)
}

fn new_game(mut p1: Deck, mut p2: Deck) -> (Deck, Deck) {
    let mut history: Vec<Deck> = Vec::new();
    let mut last = p1.clone();
    while p1.len() > 0 && p2.len() > 0 {
        if history.contains(&p1) {
            let p1c = p1.pop().unwrap();
            let p2c = p2.pop().unwrap();
            p1.insert(0, p1c);
            p1.insert(0, p2c);
            history.push(p1.clone());
            continue;
        }

        let mut p1_win = winner(&p1, &p2);

        let p1c = p1.pop().unwrap();
        let p2c = p2.pop().unwrap();


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
        history.push(last);
        last = p1.clone();
    }

    (p1, p2)
}

fn winner(p1: &Deck, p2: &Deck) -> bool {
    if p1[p1.len() - 1] > p2[p2.len() - 1] {
        return true;
    }
    false
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

    let mut score = 0;
    for (i, card) in winner.iter().enumerate() {
        score += card * (i + 1) as u32;
    }
    score
}

fn build_deck(deck: &str) -> Deck {
    let mut deck = deck.split("\n").collect::<Vec<&str>>();
    deck.reverse();
    deck.pop();
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
