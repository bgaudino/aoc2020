use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut instructions: Vec<(&str, i32)> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let instruction = parts[0];
        let n = parts[1].parse::<i32>().unwrap();
        instructions.push((instruction, n));
    }
    let mut i: i32 = 0;
    let mut accum: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    loop {
        if visited.contains(&i) {
            break;
        }
        visited.insert(i);
        let (instruction, n) = instructions[i as usize];
        match instruction {
            "acc" => {
                i += 1;
                accum += n;
            }
            "jmp" => {
                i += n;
            }
            "nop" => {
                i += 1;
            }
            _ => panic!("Invalid instruction"),
        };
    }
    println!("Part 1: {}", accum);
}
