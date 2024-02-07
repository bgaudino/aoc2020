use num::integer::lcm;
use std::{f32::INFINITY, fs};

struct Bus(usize);

impl Bus {
    fn wait_time(&self, timestamp: usize) -> usize {
        (timestamp / self.0 + 1) * self.0 - timestamp
    }

    fn has_departure_at(&self, timestamp: usize) -> bool {
        timestamp % self.0 == 0
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let (buses, timestamp) = parse(contents);
    let mut bus_id = 0;
    let mut min_wait_time = INFINITY as usize;
    for bus in &buses {
        if let Some(b) = bus {
            let wait_time = b.wait_time(timestamp);
            if wait_time < min_wait_time {
                min_wait_time = wait_time;
                bus_id = b.0;
            }
        }
    }
    let part1 = bus_id * min_wait_time;
    assert_eq!(part1, 4938);
    println!("Part 1: {}", part1);

    let mut interval: usize = 1;
    let mut part2: usize = 0;
    loop {
        let mut is_target = true;
        for (j, bus) in buses.iter().enumerate() {
            if let Some(b) = bus {
                if !b.has_departure_at(part2 + j) {
                    is_target = false;
                    break;
                } else {
                    interval = lcm(interval, b.0);
                }
            }
        }
        if is_target {
            break;
        }
        part2 += interval;
    }
    assert_eq!(part2, 230903629977901);
    println!("Part 2: {}", part2);
}

fn parse(s: String) -> (Vec<Option<Bus>>, usize) {
    let parts: Vec<&str> = s.lines().collect();
    let timestamp: usize = parts[0].parse().unwrap();
    let mut buses: Vec<Option<Bus>> = vec![];
    for part in parts[1].split(",") {
        match part.parse::<usize>() {
            Ok(v) => buses.push(Some(Bus(v))),
            _ => buses.push(None),
        }
    }
    (buses, timestamp)
}
