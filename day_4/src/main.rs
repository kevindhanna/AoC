use std::io::Read;

struct Property {
    key: String,
    value: String
}

struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

const PASSPORT_PROPERTIES: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    // "cid",
];

const EYE_COLOURS: [&str; 7] = [
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth",
];

const HEX_DIGITS: [char; 16] = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
];

fn read_file(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    contents
}

fn parse_properties(mut passport: String) -> Vec<Property> {
    passport = passport.trim().replace("\n", " ");
    let properties: Vec<&str> = passport.split(" ").collect();
    let mut parsed: Vec<Property> = Vec::new();
    for property in properties {
        let property: Vec<&str> = property.split(":").collect();
        parsed.push(Property {
            key: property[0].to_string(),
            value: property[1].to_string()
        });
    }

    parsed
}

fn build_passport(properties: Vec<Property>) -> Passport {
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
    };
    for prop in properties {
        match prop.key.as_str() {
            "byr" => passport.byr = Some(prop.value.parse::<u32>().unwrap()),
            "iyr" => passport.iyr = Some(prop.value.parse::<u32>().unwrap()),
            "eyr" => passport.eyr = Some(prop.value.parse::<u32>().unwrap()),
            "hgt" => passport.hgt = Some(prop.value),
            "hcl" => passport.hcl = Some(prop.value),
            "ecl" => passport.ecl = Some(prop.value),
            "pid" => passport.pid = Some(prop.value),
            &_ => continue,
        }
    };

    passport
}

fn validate_passport(properties: Vec<Property>) -> Result<(), ()> {
    let passport = build_passport(properties);

    if let Some(byr) = passport.byr {
        if byr < 1920 || byr > 2002 {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let Some(iyr) = passport.iyr {
        if iyr < 2010 || iyr > 2020 {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let Some(eyr) = passport.eyr {
        if eyr < 2020 || eyr > 2030 {
            return Err(());
        }
    } else {
        return Err(());
    }

    match validate_hgt(passport.hgt) {
        Err(_) => return Err(()),
        Ok(_) => {},
    }
    match validate_hcl(passport.hcl) {
        Err(_) => return Err(()),
        Ok(_) => {},
    }
    match validate_ecl(passport.ecl) {
        Err(_) => return Err(()),
        Ok(_) => {},
    }
    match validate_pid(passport.pid) {
        Err(_) => return Err(()),
        Ok(_) => {},
    }

    Ok(())
}

fn validate_hgt(hgt: Option<String>) -> Result<(),()> {
    if let Some(mut hgt) = hgt {
        if hgt.contains("cm") {
            hgt = hgt.replace("cm", "");
            let hgt: u32 = hgt.parse::<u32>().unwrap();
            if hgt < 150 || hgt > 193 {
                return Err(());
            }
            return Ok(());
        }
        if hgt.contains("in") {
            hgt = hgt.replace("in", "");
            let hgt: u32 = hgt.parse::<u32>().unwrap();
            if hgt < 59 || hgt > 76 {
                return Err(());
            }
            return Ok(());
        }
    }

    Err(())
}

fn validate_hcl(hcl: Option<String>) -> Result<(),()> {
    if let Some(mut hcl) = hcl {
        if !hcl.contains("#") {
            return Err(());
        }
        hcl = hcl.replace("#", "");

        let chars: Vec<char> = hcl.chars().collect();
        for digit in chars {
            if !HEX_DIGITS.contains(&digit) {
                println!("{}", digit);
                return Err(());
            }
        }
        return Ok(());
    }
    Err(())
}

fn validate_ecl(ecl: Option<String>) -> Result<(),()> {
    if let Some(ecl) = ecl {
        if EYE_COLOURS.contains(&ecl.as_str()) {
            return Ok(());
        }
    }
    Err(())
}

fn validate_pid(pid: Option<String>) -> Result<(),()> {
    if let Some(pid) = pid {
        if pid.len() != 9 {
            return Err(())
        }
        match pid.parse::<u32>() {
            Ok(val) => {
                if val < 1_000_000_000 {
                    return Ok(())
                }
            },
            Err(_) => {}
        }
    }
    Err(())
}

fn validate_properties(properties: &Vec<Property>) -> Result<(), ()> {
    let mut keys: Vec<&str> = Vec::new();
    for prop in properties {
        keys.push(prop.key.as_str());
    }

    if keys.len() < PASSPORT_PROPERTIES.len() {
        return Err(());
    }
    for prop in PASSPORT_PROPERTIES.to_vec() {
        if !keys.contains(&prop) {
            return Err(());
        }
    }

    Ok(())
}

fn part_1(passports: &Vec<&str>) -> u32 {
    let mut count: u32 = 0;
    for passport in passports {
        let parsed_properties = parse_properties(passport.to_string());
        match validate_properties(&parsed_properties) {
            Ok(_) => count += 1,
            Err(_) => continue
        }
    }

    count
}

fn part_2(passports: &Vec<&str>) -> u32 {
    let mut count: u32 = 0;
    for passport in passports {
        let properties: Vec<Property> = parse_properties(passport.to_string());
        match validate_properties(&properties) {
            Ok(_) => {},
            Err(_) => continue
        }
        match validate_passport(properties) {
            Ok(_) => count += 1,
            Err(_) => continue,
        }
    }

    count
}

fn main() {

    let contents = read_file("input.txt");
    let passports: Vec<&str> = contents.split("\n\n").collect();

    let part_1_result = part_1(&passports);
    let part_2_result = part_2(&passports);

    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);
}
