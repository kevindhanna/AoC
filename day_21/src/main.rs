use std::io::Read;
use std::collections::HashMap;

#[derive(Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>
}

fn main() {
    let contents = read_file("input.txt");
    let foods = contents.trim().split("\n").map(|food| build_food(food)).collect::<Vec<Food>>();

    part_1_test();
    let (part_1_result, allergens) = part_1(&foods);
    println!("part 1: {}", part_1_result);

    let part_2_result = part_2(allergens);
    println!("part 2:");
    println!("{}", part_2_result);
}

fn part_2(allergens: HashMap<String, String>) -> String {
    let mut list = "".to_owned();

    let mut sorted_allergens = allergens.keys().map(|i| i.clone()).collect::<Vec<String>>();
    sorted_allergens.sort();
    for a in sorted_allergens {
        if let Some(ingred) = allergens.get(&a) {
            list.push_str(ingred.as_str());
            list.push(',');
        }
    }
    list
}

fn part_1(foods: &Vec<Food>) -> (u32, HashMap<String, String>) {
    let (allergen_ingredients, mut translated_allergens) = build_candidates(foods);
    let mut allergens: Vec<String> = allergen_ingredients.keys().map(|a| a.clone()).collect::<Vec<String>>();

    while allergens.len() > 0 {
        let allergen = allergens.pop().unwrap();
        match translated_allergens.get(&allergen) {
            Some(_) => continue,
            None => {
                match allergen_ingredients.get(&allergen) {
                    Some(ingreds) => {
                        let mut filtered = ingreds.clone();
                        for t in translated_allergens.values() {
                            if let Some(i) = filtered.iter().position(|i| i == t) {
                                filtered.remove(i);
                            }
                        }
                        if filtered.len() == 1 {
                            translated_allergens.insert(allergen, filtered[0].clone());
                        } else {
                            allergens.insert(0, allergen);
                        }
                    },
                    None => {}
                }
            }
        }
    }

    let ingredients = list_ingredients(foods);
    let mut count = 0;
    let allergens = translated_allergens.values().map(|i| i.clone()).collect::<Vec<String>>();

    for i in ingredients {
        if allergens.contains(i) {
            continue;
        }
        count += 1;
    }
    (count, translated_allergens)
}

fn list_ingredients(foods: &Vec<Food>) -> Vec<&String> {
    let mut ingredients: Vec<&String> = Vec::new();
    for food in foods {
        for ingredient in &food.ingredients {
            ingredients.push(ingredient);
        }
    }
    ingredients
}

fn build_candidates(foods: &Vec<Food>) -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
    let mut allergen_ingredients: HashMap<String, Vec<String>> = HashMap::new();
    let mut translated_allergens: HashMap<String, String> = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            match allergen_ingredients.get(allergen) {
                Some(v) => {
                    let ingredients = reduce(v, &food.ingredients);
                    if ingredients.len() > 1 {
                        allergen_ingredients.insert(allergen.clone(), ingredients);
                    } else {
                        allergen_ingredients.remove(allergen);
                        translated_allergens.insert(allergen.clone(), ingredients[0].clone());
                    }
                },
                None => {
                    allergen_ingredients.insert(allergen.clone(), food.ingredients.clone());
                }
            }
        }
    }
    (allergen_ingredients, translated_allergens)
}

fn reduce(left: &Vec<String>, right: &Vec<String>) -> Vec<String> {
    let mut reduced: Vec<String> = Vec::new();
    for val in left {
        if right.contains(&val) {
            reduced.push(val.clone());
        }
    }
    reduced
}

fn part_1_test() {
    let contents = read_file("test_input.txt");
    let foods = contents.trim().split("\n").map(|food| build_food(food)).collect::<Vec<Food>>();
    let (result, _) = part_1(&foods);
    assert_eq!(result, 5);
}

fn build_food(string: &str) -> Food {
    let string = string.replace(")", "");

    let food = string.split(" (contains ").collect::<Vec<&str>>();
    return Food {
        ingredients: food[0].split(" ").map(|s| s.to_owned()).collect::<Vec<String>>(),
        allergens: food[1].split(", ").map(|s| s.to_owned()).collect::<Vec<String>>()
    }
}

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}
