use std::fs;

struct Validator {
    ch: char,
    min: usize,
    max: usize,
    password: String,
}

impl Validator {
    fn is_valid(&self) -> bool {
        let mut count: usize = 0;
        for ch in self.password.chars() {
            if ch == self.ch {
                count += 1;
            }
        }
        count >= self.min && count <= self.max
    }

    fn is_valid_part2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let min = chars[self.min - 1];
        let max = chars[self.max - 1];
        let mut matches: u8 = 0;
        if min == self.ch {
            matches += 1;
        }
        if max == self.ch {
            matches += 1;
        }
        matches == 1
    }

    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();
        let digits: Vec<&str> = parts[0].split("-").collect();
        let min = digits[0].parse::<usize>().unwrap();
        let max = digits[1].parse::<usize>().unwrap();
        let str_vec: Vec<&str> = parts[1].split("-").collect();
        let ch_vec: Vec<char> = str_vec[0].chars().collect();
        let ch = ch_vec[0];
        let password = parts[2];
        Validator {
            ch: ch,
            min: min,
            max: max,
            password: String::from(password),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for line in lines {
        let validator = Validator::new(line);
        if validator.is_valid() {
            part1 += 1;
        }
        if validator.is_valid_part2() {
            part2 += 1
        }
    }
    assert_eq!(part1, 655);
    println!("Part 1: {}", part1);
    assert_eq!(part2, 673);
    println!("Part 2: {}", part2);
}
