use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        n >= self.min && n <= self.max
    }

    fn parse(s: &str) -> Self {
        let parts = s.split("-").collect::<Vec<&str>>();
        Self {
            min: parts[0].parse::<usize>().unwrap(),
            max: parts[1].parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<usize>,
}

impl Ticket {
    fn parse(s: &str) -> Self {
        Self {
            numbers: s.split(",").map(|n| n.parse::<usize>().unwrap()).collect(),
        }
    }
}

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Self {
            numbers: self.numbers.clone(),
        }
    }
}

fn get_error_rate(ticket: &Ticket, rules: &HashMap<&str, (Range, Range)>) -> (bool, usize) {
    'outer: for n in &ticket.numbers {
        for (_, (r1, r2)) in rules {
            if r1.contains(*n) || r2.contains(*n) {
                continue 'outer;
            }
        }
        return (true, *n);
    }
    (false, 0)
}

fn main() {
    let mut rules: HashMap<&str, (Range, Range)> = HashMap::new();
    let contents: String = fs::read_to_string("data.txt").unwrap();
    let mut step = 0;
    let mut part1: usize = 0;
    let mut valid_tickets: Vec<Ticket> = Vec::new();
    let mut possible: HashMap<&str, HashSet<usize>> = HashMap::new();
    let mut my_ticket = Ticket {
        numbers: Vec::new(),
    };
    for line in contents.lines() {
        if line == "" {
            continue;
        }
        if line == "your ticket:" || line == "nearby tickets:" {
            step += 1;
            continue;
        }
        if step == 0 {
            let parts: Vec<&str> = line.split(":").collect();
            let name = parts[0];
            let mut s: HashSet<usize> = HashSet::new();
            for i in 0..20 {
                s.insert(i);
            }
            possible.insert(name, s);
            let ranges: Vec<&str> = parts[1].split_whitespace().collect();
            rules.insert(name, (Range::parse(ranges[0]), Range::parse(ranges[2])));
        } else if step == 1 {
            my_ticket = Ticket::parse(line);
        } else if step == 2 {
            let ticket = Ticket::parse(line);
            let (error, error_rate) = get_error_rate(&ticket, &rules);
            if error {
                part1 += error_rate;
            } else {
                valid_tickets.push(ticket);
            }
        }
    }
    assert_eq!(part1, 27898);
    println!("Part 1: {}", part1);

    let mut solved: HashMap<&str, usize> = HashMap::new();
    let mut taken: HashSet<usize> = HashSet::new();
    while taken.len() < 20 {
        for i in 0..20 {
            if taken.contains(&i) {
                continue;
            }
            let mut possible: HashSet<&str> = HashSet::new();
            'outer: for (rule, (r1, r2)) in &rules {
                if solved.contains_key(rule) {
                    continue;
                }
                for ticket in &valid_tickets {
                    let n = ticket.numbers[i];
                    if !(r1.contains(n) || r2.contains(n)) {
                        continue 'outer;
                    }
                }
                possible.insert(&rule);
            }
            if possible.len() == 1 {
                for p in &possible {
                    solved.insert(p, i);
                    taken.insert(i);
                }
            }
        }
    }
    
    let mut part2: usize = 1;
    for (rule, p) in &solved {
        if rule.starts_with("departure") {
            part2 *= my_ticket.numbers[*p];
        }
    }
    assert_eq!(part2, 2766491048287);
    println!("Part 2: {}", part2);
}
