use std::fs;

fn evalutate(mut eq: String) -> usize {
    fn _evaluate(eq: &String) -> usize {
        let mut operators: Vec<&str> = Vec::new();
        let mut integers: Vec<usize> = Vec::new();
        for s in eq.split_whitespace() {
            if s == "*" || s == "+" {
                operators.push(s);
            } else {
                integers.push(s.parse::<usize>().unwrap());
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
                        let e = _evaluate(s);
                        let mut before = eq[..start].to_string();
                        before.push_str(e.to_string().as_str());
                        before.push_str(&eq[i + 1..]);
                        eq = before;
                        break;
                    }
                }
            }
            None => break,
        }
    }
    _evaluate(&eq)
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut part1: usize = 0;
    for line in contents.lines() {
        part1 += evalutate(String::from(line));
    }
    println!("Part 1: {}", part1);
}
