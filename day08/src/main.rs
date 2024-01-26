use std::collections::HashSet;
use std::fs;

fn run(instructions: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut i: i32 = 0;
    let mut accum: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut terminates = false;
    loop {
        if visited.contains(&i) {
            break;
        }
        if i as usize >= instructions.len() {
            terminates = true;
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
    (accum, terminates)
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut instructions: Vec<(&str, i32)> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let instruction = parts[0];
        let n = parts[1].parse::<i32>().unwrap();
        instructions.push((instruction, n));
    }
    let (part1, _) = run(&instructions);
    println!("Part 1: {}", part1);

    for i in 0..instructions.len() {
        let (instruction, n) = instructions[i];
        if instruction == "nop" || instruction == "jmp" {
            let new_instruction = if instruction == "nop" { "jmp" } else { "nop" };
            instructions[i] = (new_instruction, n);
            let (accum, terminates) = run(&instructions);
            if terminates {
                println!("Part 2: {}", accum);
                break;
            } else {
                instructions[i] = (instruction, n);
            }
        }
    }
}
