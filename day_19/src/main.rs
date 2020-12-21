use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let contents = read_file("input.txt");
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let rules = parts[0].split('\n')
                        .map(|r| {
                            return r.replace("\"", "");
                        })
                        .collect::<Vec<String>>();
    let messages = parts[1].split('\n').collect::<Vec<&str>>();

    part_1_test();
    let part_1_result = part_1(&messages, &rules);
    println!("part 1: {}", part_1_result);

    part_2_test();
    let part_2_result = part_2(&messages, &rules);
    println!("part 2: {}", part_2_result);
}

fn part_2(messages: &Vec<&str>, rules: &Vec<String>) -> u32 {
    // {N} instead of {1,} because they have to be the same length!
    let rules = replace_rule("11: 42 31", "11: 42 31 | 42 {N} 31 {N}", &rules);
    let rules = replace_rule("8: 42", "8: 42 | 42 {1,}", &rules);
    let rules = parse_rules(rules);

    let mut regex = rules.get("0").unwrap().clone();
    regex.push_str("$");
    regex.insert_str(0, "^");


    let mut matched: Vec<&str> = Vec::new();
    for i in 1..10 {
        let regex = regex.replace("N", i.to_string().as_str());
        let re = Regex::new(regex.as_str()).unwrap();
        for message in messages {
            if re.is_match(message) {
                matched.push(message);
            }
        }
    }

    let mut dedup: Vec<&str> = Vec::new();
    for m in matched {
        if dedup.contains(&m) {
            continue
        }
        dedup.push(m);
    }
    dedup.len() as u32
}

fn replace_rule(old: &str, new: &str, rules: &Vec<String>) -> Vec<String> {
    let mut rules = rules.clone();
    match rules.iter().position(|r| r == old) {
        Some(i) => {
            rules[i] = new.to_owned();
        },
        None => {}
    }
    rules
}

fn part_1(messages: &Vec<&str>, rules: &Vec<String>) -> u32 {
    let rules = parse_rules(rules.clone());

    let mut regex = rules.get("0").unwrap().clone();
    regex.push_str("$");
    regex.insert_str(0, "^");
    let re = Regex::new(regex.as_str()).unwrap();

    let mut count = 0;
    for message in messages {
        if re.is_match(message) {
            count += 1
        }
    }
    count
}

fn parse_rules (mut rules: Vec<String>) -> HashMap<String, String> {
    let mut parsed: HashMap<String, String> = HashMap::new();
    while parsed.len() < 2 {
        for i in 0..rules.len() {
            // pre populate the a and b
            if rules[i].contains("a") || rules[i].contains("b") {
                let rule = rules[i].split(": ").collect::<Vec<&str>>();
                parsed.insert(rule[0].to_owned(), rule[1].to_owned());
                // remove them so we don't try to process them again later
                rules.remove(i);
                break;
            }
        }
    }
    while rules.len() > 0 {
        let popped = rules.pop().unwrap();
        let rule = popped.split(": ").collect::<Vec<&str>>();

        // initialise the parsed rule so we can append stuff to it
        let mut p_rule = "".to_owned();
        // 12 15 | 10 5 => [12, 15, |, 10, 5]
        let subs = rule[1].split(" ").collect::<Vec<&str>>();
        for (i, sub) in subs.iter().enumerate() {
            if sub == &"|" {
                p_rule.push_str(sub);
                continue;
            }

            if sub == &rule[0] {
                continue;
            }
            // handle part 2
            if sub == &"{1,}" || sub == &"{N}" {
                p_rule.push_str(sub);
                // check if we've processed all the sub rules
                if i == subs.len() - 1 {
                    p_rule.push_str(")");
                    p_rule.insert_str(0, "(");
                    parsed.insert(rule[0].into(), p_rule.clone().into());
                }
            } else {
                // can we parse this subrule?
                match parsed.get(sub.clone()) {
                    Some(val) => {
                        p_rule.push_str(val);
                        // check if we've processed all the sub rules
                        if i == subs.len() - 1 {
                            p_rule.push_str(")");
                            p_rule.insert_str(0, "(");
                            parsed.insert(rule[0].into(), p_rule.clone().into());
                        }
                    },
                    None => {
                        // put unfinished rule back into the stack;
                        rules.insert(0, popped);
                        break;
                    }
                }
            }
        }
    }

    parsed
}

fn part_1_test() {
    let rules = vec![
        "0: 4 1 5".to_owned(),
        "1: 2 3 | 3 2".to_owned(),
        "2: 4 4 | 5 5".to_owned(),
        "3: 4 5 | 5 4".to_owned(),
        "4: a".to_owned(),
        "5: b".to_owned()
    ];
    let messages = vec![
        "ababbb",
        "bababa",
        "abbbab",
        "aaabbb",
        "aaaabbb"
    ];
    assert_eq!(part_1(&messages, &rules), 2);
}

fn part_2_test() {
    let rules = vec![
        "42: 9 14 | 10 1".to_owned(),
        "9: 14 27 | 1 26".to_owned(),
        "10: 23 14 | 28 1".to_owned(),
        "1: a".to_owned(),
        "11: 42 31".to_owned(),
        "5: 1 14 | 15 1".to_owned(),
        "19: 14 1 | 14 14".to_owned(),
        "12: 24 14 | 19 1".to_owned(),
        "16: 15 1 | 14 14".to_owned(),
        "31: 14 17 | 1 13".to_owned(),
        "6: 14 14 | 1 14".to_owned(),
        "2: 1 24 | 14 4".to_owned(),
        "0: 8 11".to_owned(),
        "13: 14 3 | 1 12".to_owned(),
        "15: 1 | 14".to_owned(),
        "17: 14 2 | 1 7".to_owned(),
        "23: 25 1 | 22 14".to_owned(),
        "28: 16 1".to_owned(),
        "4: 1 1".to_owned(),
        "20: 14 14 | 1 15".to_owned(),
        "3: 5 14 | 16 1".to_owned(),
        "27: 1 6 | 14 18".to_owned(),
        "14: b".to_owned(),
        "21: 14 1 | 1 14".to_owned(),
        "25: 1 1 | 1 14".to_owned(),
        "22: 14 14".to_owned(),
        "8: 42".to_owned(),
        "26: 14 22 | 1 20".to_owned(),
        "18: 15 15".to_owned(),
        "7: 14 5 | 1 21".to_owned(),
        "24: 14 1".to_owned(),
    ];
    let messages = vec![
        "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
        "bbabbbbaabaabba",
        "babbbbaabbbbbabbbbbbaabaaabaaa",
        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
        "bbbbbbbaaaabbbbaaabbabaaa",
        "bbbababbbbaaaaaaaabbababaaababaabab",
        "ababaaaaaabaaab",
        "ababaaaaabbbaba",
        "baabbaaaabbaaaababbaababb",
        "abbbbabbbbaaaababbbbbbaaaababb",
        "aaaaabbaabaaaaababaa",
        "aaaabbaaaabbaaa",
        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
        "babaaabbbaaabaababbaabababaaab",
        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    ];
    assert_eq!(part_2(&messages, &rules), 12);
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
