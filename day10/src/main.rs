use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let mut adapters: Vec<u32> = content
        .lines()
        .map(|n: &str| n.parse::<u32>().unwrap())
        .collect();
    adapters.sort();
    let mut differences: HashMap<u32, u32> = HashMap::from([(3, 1)]);
    let mut current: u32 = 0;
    for adapter in adapters {
        let difference = adapter - current;
        let count = differences.get(&difference).unwrap_or(&0);
        differences.insert(difference, count + 1);
        current = adapter;
    }
    let part1 = differences.get(&1).unwrap() * differences.get(&3).unwrap();
    println!("Part 1: {}", part1);
}
