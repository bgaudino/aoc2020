use std::collections::HashMap;
use std::fs;
extern crate regex;
use regex::Regex;

fn get_year(s: &str) -> Option<i32> {
    let re = Regex::new("^\\d{4}$").unwrap();
    if re.is_match(s) {
        let n = s.parse::<i32>().unwrap();
        return Some(n);
    }
    None
}

const FIELDS: [(&str, fn(&str) -> bool); 7] = [
    ("byr", |v: &str| -> bool {
        let year = get_year(v);
        match year {
            Some(y) => return y >= 1920 && y <= 2002,
            None => return false,
        };
    }),
    ("iyr", |v: &str| -> bool {
        let year = get_year(v);
        match year {
            Some(y) => return y >= 2010 && y <= 2020,
            None => return false,
        };
    }),
    ("eyr", |v: &str| -> bool {
        let year = get_year(v);
        match year {
            Some(y) => return y >= 2020 && y <= 2030,
            None => return false,
        };
    }),
    ("hgt", |v: &str| -> bool {
        let re = Regex::new("^\\d+(cm|in)$").unwrap();
        if !re.is_match(v) {
            return false;
        }
        let s = String::from(v);
        let n = s[..s.len() - 2].parse::<i32>().unwrap();
        if v.ends_with("cm") {
            return n >= 150 && n <= 193;
        }
        n >= 59 && n <= 76
    }),
    ("hcl", |v: &str| -> bool {
        Regex::new("^#[0-9a-f]{6}").unwrap().is_match(v)
    }),
    ("ecl", |v: &str| -> bool {
        Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$")
            .unwrap()
            .is_match(v)
    }),
    ("pid", |v: &str| -> bool {
        Regex::new("^\\d{9}$").unwrap().is_match(v)
    }),
];

fn is_valid(passport: &HashMap<&str, &str>, validate_fields: bool) -> bool {
    for field in FIELDS {
        if !passport.contains_key(field.0) {
            return false;
        }
        if validate_fields && !field.1(passport.get(field.0).unwrap()) {
            return false;
        }
    }
    true
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let passports = contents.split("\n\n");
    let mut part1 = 0;
    let mut part2 = 0;
    for passport in passports {
        let mut map: HashMap<&str, &str> = HashMap::new();
        let components: Vec<&str> = passport.split_whitespace().collect();
        for component in components {
            let value: Vec<&str> = component.split(":").collect();
            map.insert(value[0], value[1]);
        }
        if is_valid(&map, false) {
            part1 += 1;
        }
        if is_valid(&map, true) {
            part2 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
