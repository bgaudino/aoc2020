use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let groups = contents.split("\n\n");
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for group in groups {
        let mut questions: HashMap<char, usize> = HashMap::new();
        let people: Vec<&str> = group.split("\n").collect();
        for person in &people {
            for ch in person.chars() {
                let count = questions.get(&ch);
                match count {
                    Some(c) => questions.insert(ch, *c + 1),
                    None => questions.insert(ch, 1),
                };
            }
        }
        for (_, count) in &questions {
            part1 += 1;
            if *count == people.len() {
                part2 += 1;
            }
        }
    }
    assert_eq!(part1, 6457);
    println!("Part 1: {}", part1);
    assert_eq!(part2, 3260);
    println!("Part 2: {}", part2);
}
