use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let report: Vec<u32> = contents
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let part1 = product_of_two_part_sum(&report);
    assert_eq!(part1, 918339);
    println!("Part 1: {}", part1);
    let part2 = product_of_three_part_sum(&report);
    assert_eq!(part2, 23869440);
    println!("Part 2: {}", part2);
}

fn product_of_two_part_sum(report: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    for (i, entry_a) in report.iter().enumerate() {
        for entry_b in &report[i + 1..] {
            if entry_a + entry_b == 2020 {
                result = entry_a * entry_b;
                break;
            }
        }
    }
    result
}

fn product_of_three_part_sum(report: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    for (i, entry_a) in report.iter().enumerate() {
        for (j, entry_b) in report[i + 1..].iter().enumerate() {
            for entry_c in &report[j + 1..] {
                if entry_a + entry_b + entry_c == 2020 {
                    result = entry_a * entry_b * entry_c;
                    break;
                }
            }
        }
    }
    result
}
