use std::fs;

fn evalutate(mut eq: String, is_advanced: bool) -> usize {
    fn _evaluate(eq: &String, is_advanced: bool) -> usize {
        let mut operators: Vec<&str> = Vec::new();
        let mut integers: Vec<usize> = Vec::new();
        for s in eq.split_whitespace() {
            if s == "*" || s == "+" {
                operators.push(s);
            } else {
                integers.push(s.parse::<usize>().unwrap());
            }
        }
        if is_advanced {
            'outer: loop {
                for i in 0..operators.len() {
                    let operator = operators[i];
                    if operator == "+" {
                        let num = integers[i] + integers[i + 1];
                        let mut new_operators = operators[..i].to_vec();
                        new_operators.extend_from_slice(&operators[i + 1..]);
                        let mut new_integers = integers[..i].to_vec();
                        new_integers.push(num);
                        new_integers.extend_from_slice(&integers[i + 2..]);
                        operators = new_operators;
                        integers = new_integers;
                        continue 'outer;
                    }
                }
                break;
            }
        }
        let mut n = integers[0];
        for i in 0..operators.len() {
            let operator = operators[i];
            if operator == "*" {
                n *= integers[i + 1];
            } else if operator == "+" {
                n += integers[i + 1];
            }
        }
        n
    }
    loop {
        match eq.rfind("(") {
            Some(start) => {
                for (i, ch) in eq.chars().enumerate() {
                    if i > start && ch == ')' {
                        let s = &eq[start + 1..i].to_string();
                        let e = _evaluate(s, is_advanced);
                        let mut new_eq = eq[..start].to_string();
                        new_eq.push_str(e.to_string().as_str());
                        new_eq.push_str(&eq[i + 1..]);
                        eq = new_eq;
                        break;
                    }
                }
            }
            None => break,
        }
    }
    _evaluate(&eq, is_advanced)
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for line in contents.lines() {
        part1 += evalutate(String::from(line), false);
        part2 += evalutate(String::from(line), true);
    }
    assert_eq!(part1, 12918250417632);
    println!("Part 1: {}", part1);
    assert_eq!(part2, 171259538712010);
    println!("Part 2: {}", part2);
}
