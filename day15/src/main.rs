use std::collections::HashMap;

fn main() {
    let starting_numbers: Vec<usize> = vec![18, 8, 0, 5, 4, 1, 20];
    let l = starting_numbers.len();
    let mut spoken: Vec<usize> = Vec::new();
    let mut numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..30000000 {
        let n: usize = {
            if i < l {
                starting_numbers[i]
            } else {
                match numbers.get(spoken.last().unwrap()) {
                    Some(indicies) => {
                        let l = indicies.len();
                        if l > 1 {
                            indicies[l - 1] - indicies[l - 2]
                        } else {
                            0
                        }
                    }
                    None => 0,
                }
            }
        };
        spoken.push(n);
        match numbers.get_mut(&n) {
            Some(v) => {
                v.push(i);
            }
            None => {
                numbers.insert(n, vec![i]);
            }
        };
    }
    let part1 = spoken[2019];
    // TODO: optimize. Brute force takes about 15 seconds
    let part2 = spoken.last().unwrap();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
