use std::collections::HashSet;
use std::fs;

const GROUP_SIZE: usize = 25;

fn main() {
    let nums: Vec<usize> = fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut group: HashSet<usize> = HashSet::new();
    let mut part1: usize = 0;
    for (i, num) in nums.iter().enumerate() {
        if i >= GROUP_SIZE {
            let mut found = false;
            for n in &group {
                if n > num {
                    continue;
                }
                let compliment = num - n;
                if compliment != *n && group.contains(&compliment) {
                    found = true;
                    break;
                }
            }
            if !found {
                part1 = *num;
                break;
            }
            group.remove(&nums[i - GROUP_SIZE]);
        }
        group.insert(*num);
    }
    println!("Part 1: {}", part1);

    let mut part2: usize = 0;
    'outer: for (i, num) in nums.iter().enumerate() {
        let mut sum: usize = *num;
        let mut min = *num;
        let mut max = *num;
        for n in &nums[i + 1..] {
            sum += *n;
            if *n > max {
                max = *n;
            } else if *n < min {
                min = *n;
            }
            if sum == part1 {
                part2 = min + max;
                break 'outer;
            }
            if sum > part1 {
                break;
            }
        }
    }
    println!("Part 2: {}", part2);
}
