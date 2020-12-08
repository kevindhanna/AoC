use std::io::Read;
use std::time::Instant;

#[derive(Clone)]
struct Node {
    name: String,
    count: u32,
    children: Vec<Node>
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn build_trie(rules: Vec<String>) -> Vec<Node> {
    let mut bags: Vec<Node> = Vec::new();
    for rule in rules {
        let rule: Vec<&str> = rule.split(":").collect::<Vec<&str>>();
        bags.push(build_base_node(rule));
    }
    let trie: Vec<Node> = bags.iter().map(|bag| populate_children(bag.clone(), &bags))
                                     .collect();
    trie
}

fn build_base_node(rule: Vec<&str>) -> Node {
    let mut children: Vec<Node> = Vec::new();
    if rule.len() > 1 {
        let child_rules: Vec<&str> = rule[1].split(",").collect();
        for child in child_rules {
            let parts: Vec<&str> = child.split(" ").collect();
            let name: String = parts[1..].join(" ");
            let count: u32 = parts[0].parse::<u32>().unwrap();
            children.push(Node {
                name,
                count,
                children: Vec::new()
            })
        }
    }
    Node {
        name: rule[0].into(),
        count: 1,
        children: children
    }
}

fn populate_children(mut node: Node, bags: &Vec<Node>) -> Node {
    let old_children = node.children;
    let mut new_children = Vec::new();
    for child in old_children {
        if let Some(mut new) = find(bags, child.name.as_str()) {
            new.count = child.count;
            new_children.push(populate_children(new, bags));
        } else {
            new_children.push(populate_children(child, bags));
        }
    }

    node.children = new_children;
    node
}

fn find(trie: &Vec<Node>, name: &str) -> Option<Node> {
    for node in trie {
        if node.name.as_str() == name {
            return Some(node.clone())
        }
    }
    None
}

fn part_1(trie: &Vec<Node>) -> u32 {
    let mut count = 0;
    for node in trie {
        if node.name != "shiny gold" && can_hold(node.clone(), "shiny gold") {
            count += 1;
        }
    }
    count
}

fn can_hold(node: Node, colour: &str) -> bool {
    if node.name.as_str() == colour {
        return true;
    }
    for child in node.children {
        if can_hold(child, colour) {
            return true
        }
    }
    false
}

fn part_2(trie: &Vec<Node>) -> Option<u32> {
    if let Some(shiny_gold) = find(trie, "shiny gold") {
        return Some(calculate_total(shiny_gold) - 1);
    }
    None
}

fn calculate_total(node: Node) -> u32 {
    if node.children.len() == 0 {
        return node.count
    }
    let mut child_count = 0;
    for child in node.children {
        child_count += calculate_total(child)
    }

    child_count * node.count + node.count
}

fn part_2_tests() {
    let dark_violet2 = build_node("dark violet", 2, Vec::new());
    let dark_blue2 = build_node("dark blue", 2, vec![dark_violet2]);
    let dark_green2 = build_node("dark green", 2, vec![dark_blue2]);
    let dark_yellow2 = build_node("dark yellow", 2, vec![dark_green2]);
    let dark_orange2 = build_node("dark orange", 2, vec![dark_yellow2]);
    let dark_red2 = build_node("dark red", 2, vec![dark_orange2]);
    let shiny_gold = build_node("shiny gold", 1, vec![dark_red2]);
    assert_eq!(calculate_total(shiny_gold), 127);
}

fn build_node(name: &str, count: u32, children: Vec<Node>) -> Node {
    Node {
        name: name.into(),
        count: count,
        children: children
    }
}

fn main() {
    let now = Instant::now();
    let contents = read_file("input.txt");
    let read = Instant::now();
    println!("read file: {:?}", read.duration_since(now));
    let rules = contents
        .trim()
        .split("\n")
        .map(|rule| rule
             .replace(" contain no other bags.", "")
             .replace(" bags.", "")
             .replace(" bag.", "")
             .replace(" bags contain ", ":")
             .replace(" bags, ", ",")
             .replace(" bag, ", ","))
        .collect::<Vec<String>>();
    let splits = Instant::now();
    println!("splits and replaces: {:?}", splits.duration_since(read));
    let trie: Vec<Node> = build_trie(rules);
    let trie_time = Instant::now();
    println!("trie: {:?}", trie_time.duration_since(splits));

    let part_1_result = part_1(&trie);
    println!("part 1: {}", part_1_result);
    let part1 = Instant::now();
    println!("part1: {:?}", part1.duration_since(trie_time));

    part_2_tests();
    if let Some(part_2_result) = part_2(&trie) {
        println!("part 2: {}", part_2_result);
    }
    let part2 = Instant::now();
    println!("part2: {:?}", part2.duration_since(part1));
    println!("total: {:?}", part2.duration_since(now));
}
