use std::{collections::HashMap, fs};

fn find_shiny_gold(key: String, map: &HashMap<String, HashMap<String, u32>>) -> bool {
    let mut queue: Vec<String> = Vec::new();
    queue.push(key);
    while queue.len() > 0 {
        let bag = queue.pop().unwrap();
        let holds = map.get(&bag).unwrap();
        if holds.contains_key(&"shiny gold".to_string()) {
            return true;
        } else {
            for (key, _) in holds {
                queue.push(key.to_string());
            }
        }
    }
    false
}

fn num_bags_contained(key: String, map: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let mut num_bags: u32 = if key == "shiny gold" { 0 } else { 1 };
    for (bag, count) in map.get(&key).unwrap() {
        num_bags += count * num_bags_contained(bag.to_string(), map);
    }
    num_bags
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut map: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for line in contents.lines() {
        let mut m: HashMap<String, u32> = HashMap::new();
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let bag = String::from(parts[0]);
        let rest: Vec<&str> = parts[1].split(", ").collect();
        for item in rest {
            let parts: Vec<&str> = item.split_whitespace().collect();
            let num = parts[0].parse::<u32>();
            match num {
                Result::Ok(count) => {
                    let name = parts[1..3].join(" ");
                    m.insert(name, count);
                }
                _ => (),
            }
        }
        map.insert(bag, m);
    }
    let mut can_hold_gold: u32 = 0;
    for (key, _) in &map {
        if find_shiny_gold(key.to_string(), &map) {
            can_hold_gold += 1;
        }
    }
    assert_eq!(can_hold_gold, 121);
    println!("Part 1: {}", can_hold_gold);

    let num_bags = num_bags_contained("shiny gold".to_string(), &map);
    assert_eq!(num_bags, 3805);
    println!("Part 2: {}", num_bags);
}
