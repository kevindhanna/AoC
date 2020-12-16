use std::io::Read;
use std::collections::HashMap;

#[derive(Clone)]
struct Rule {
    name: String,
    ranges: Vec<(u64, u64)>
}

#[derive(Clone)]
struct Value {
    val: u64,
    rules: Vec<String>
}

type Ticket = Vec<Value>;

fn main() {
    let input = read_file("input.txt");
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let (rules, my_ticket, nearby_tickets) = build_parts(parts);

    part_1_test();
    let part_1_result = part_1(&nearby_tickets, &rules);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_2(my_ticket, &nearby_tickets, &rules);
    println!("part 2: {}", part_2_result);
}

fn part_2(my_ticket: Vec<u64>, nearby_tickets: &Vec<Vec<u64>>, rules: &Vec<Rule>) -> u64 {
    let valid = valid_tickets(nearby_tickets, rules);

    let mut rule_indices: HashMap<usize, String> = HashMap::new();
    let mut missing_rule_indices = (0..my_ticket.len()).collect::<Vec<usize>>();
    while rule_indices.iter().len() < rules.len() {
        let mut missing: Vec<usize> = Vec::new();
        for i in missing_rule_indices {
            let used_rules = rule_indices.iter().map(|(i, name)| name.clone()).collect::<Vec<String>>();
            let mut vals: Vec<Value> = Vec::new();
            for ticket in &valid {
                vals.push(ticket[i].clone());
            }
            match rules_from_values(vals, rules, used_rules) {
                Ok(rules) => {
                    rule_indices.insert(i, rules[0].clone());
                },
                Err(rules) => missing.push(i)
            }
        }
        missing_rule_indices = missing;
    }
    let mut total: u64 = 1;
    for (i, val) in my_ticket.iter().enumerate() {
        match rule_indices.get(&i) {
            Some(rule) => {
                if rule.as_str().contains("departure") {
                    total *= val;
                }
            },
            None => panic!("no rule?")
        }
    }
    total
}

fn rules_from_values(values: Vec<Value>, rules: &Vec<Rule>, used_rules: Vec<String>) -> Result<Vec<String>, Vec<String>> {
    let rules = rules.iter().filter(|rule| !used_rules.contains(&rule.name)).collect::<Vec<&Rule>>();
    let mut valid_rules: Vec<String> = Vec::new();
    for rule in rules {
        let mut valid = true;
        for value in &values {
            if !value.rules.contains(&rule.name) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_rules.push(rule.name.clone());
        }
    }
    if valid_rules.len() == 1 {
        return Ok(valid_rules);
    }
    Err(valid_rules)
}

fn valid_tickets(tickets: &Vec<Vec<u64>>, rules: &Vec<Rule>) -> Vec<Ticket> {
    let mut valid: Vec<Ticket> = Vec::new();
    for ticket in tickets {
        match valid_ticket(ticket, rules) {
            Ok(validated) => valid.push(validated),
            Err(_) => continue
        }
    }
    valid
}

fn valid_ticket(ticket: &Vec<u64>, rules: &Vec<Rule>) -> Result<Ticket, ()> {
    let mut validated: Ticket = Vec::new();
    for val in ticket {
        match valid_value(val, rules) {
            Ok(rules) => validated.push(Value { val: val.clone(), rules }),
            Err(_) => return Err(())
        }
    }
    Ok(validated)
}

fn part_1(tickets: &Vec<Vec<u64>>, rules: &Vec<Rule>) -> u64 {
    let mut values: Vec<u64> = Vec::new();
    let total = sum_tickets(&tickets);
    for ticket in tickets {
        values.append(&mut valid_values(&ticket, rules));
    }
    total - values.iter().sum::<u64>()
}

fn valid_values(ticket: &Vec<u64>, rules: &Vec<Rule>) -> Vec<u64> {
    let mut values: Vec<u64> = Vec::new();
    for val in ticket {
        match valid_value(val, rules) {
            Ok(_) => values.push(*val),
            Err(_) => continue
        }
    }
    values
}

fn valid_value(val: &u64, rules: &Vec<Rule>) -> Result<Vec<String>, ()> {
    let mut valid_for: Vec<String> = Vec::new();
    for rule in rules {
        if (rule.ranges[0].0..=rule.ranges[0].1).contains(&val) ||
            (rule.ranges[1].0..=rule.ranges[1].1).contains(&val) {
                valid_for.push(rule.name.clone());
            }
    }
    match valid_for.len() > 0 {
        true => Ok(valid_for),
        false => Err(())
    }
}

fn sum_tickets(tickets: &Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    for ticket in tickets {
        for val in ticket {
            total += val;
        }
    }
    total
}

fn part_1_test() {
    let input = vec![
        "class: 1-3 or 5-7\n\
         row: 6-11 or 33-44\n\
         seat: 13-40 or 45-50",
        "your ticket:\n\
         7,1,14",
        "nearby tickets:\n\
         7,3,47\n\
         40,4,50\n\
         55,2,20\n\
         38,6,12"
    ];
    let (rules, _my_ticket, nearby_tickets) = build_parts(input);
    assert_eq!(part_1(&nearby_tickets, &rules), 71);
}

fn build_parts(parts: Vec<&str>) -> (Vec<Rule>, Vec<u64>, Vec<Vec<u64>>) {
    let rules = build_rules(parts[0]);
    let my_ticket = build_ticket(parts[1].trim().replace("your ticket:\n", "").as_str());
    let nearby_tickets = parts[2].trim()
                                 .replace("nearby tickets:\n", "")
                                 .split("\n")
                                 .map(|ticket| build_ticket(ticket))
                                 .collect::<Vec<Vec<u64>>>();
    (rules, my_ticket, nearby_tickets)
}

fn build_rules(rules: &str) -> Vec<Rule> {
    rules.split('\n').map(|rule| {
        let rule = rule.split(": ")
                       .map(|name| name.into())
                       .collect::<Vec<String>>();
        return Rule {
            name: rule[0].clone(),
            ranges: build_range_set(rule[1].clone())
        }
    }).collect::<Vec<Rule>>()
}

fn build_range_set(ranges: String) -> Vec<(u64, u64)> {
    ranges.split(" or ")
          .map(|range| {
              let range = range.split('-').collect::<Vec<&str>>();
              let start = range[0].parse::<u64>().unwrap();
              let end = range[1].parse::<u64>().unwrap();
              return (start, end)
          })
          .collect::<Vec<(u64, u64)>>()
}

fn build_ticket(ticket: &str) -> Vec<u64> {
    ticket.split(",").map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
