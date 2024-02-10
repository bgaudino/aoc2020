use std::{collections::HashMap, fs};

fn parse_mask(s: &str) -> HashMap<usize, char> {
    let mut map: HashMap<usize, char> = HashMap::new();
    for (i, ch) in s.split_at(7).1.chars().enumerate() {
        if ch != 'X' {
            map.insert(i, ch);
        }
    }
    map
}

fn parse_instruction(s: &str) -> (usize, usize) {
    let parts: Vec<&str> = s.split(" = ").collect();
    let location = {
        let part = parts[0].split_at(3);
        part.1[1..part.1.len() - 1].parse::<usize>().unwrap()
    };
    let value: usize = parts[1].parse().unwrap();
    (location, value)
}

fn get_binary(decimal: usize) -> String {
    let mut binary = format!("{:b}", decimal);
    if binary.len() < 36 {
        let padding = String::from_utf8(vec![b'0'; 36 - binary.len()]).unwrap();
        binary = padding + &binary;
    }
    binary
}

fn get_decimal(binary: String) -> usize {
    usize::from_str_radix(&binary, 2).unwrap()
}

fn mask_value(binary: &mut String, mask: &HashMap<usize, char>) {
    for (i, value) in mask {
        binary.replace_range(*i..*i + 1, value.to_string().as_str());
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask: HashMap<usize, char> = HashMap::new();
    for line in contents.lines() {
        if line.starts_with("mask") {
            mask = parse_mask(line);
        } else {
            let (location, value) = parse_instruction(line);
            let mut binary = get_binary(value);
            mask_value(&mut binary, &mask);
            memory.insert(location, get_decimal(binary));
        }
    }
    let mut part1: usize = 0;
    for (_, value) in memory {
        part1 += value;
    }
    assert_eq!(part1, 8566770985168);
    println!("Part 1: {}", part1);
}
