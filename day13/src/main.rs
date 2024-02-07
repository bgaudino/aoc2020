use std::fs;

struct Bus(usize);

impl Bus {
    fn wait_time(&self, timestamp: usize) -> usize {
        (timestamp / self.0 + 1) * self.0 - timestamp
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let (buses, timestamp) = parse(contents);
    let mut bus_id = buses[0].0;
    let mut min_wait_time = buses[0].wait_time(timestamp);
    for bus in &buses[1..] {
        let wait_time = bus.wait_time(timestamp);
        if wait_time < min_wait_time {
            min_wait_time = wait_time;
            bus_id = bus.0;
        } 
    }
    let part1 = bus_id * min_wait_time;
    println!("Part 1: {}", part1);
}

fn parse(s: String) -> (Vec<Bus>, usize) {
    let parts: Vec<&str> = s.lines().collect();
    let timestamp: usize = parts[0].parse().unwrap();
    let mut buses: Vec<Bus> = vec![];
    for part in parts[1].split(",") {
        match part.parse::<usize>() {
            Ok(v) => buses.push(Bus(v)),
            _ => (),
        }
    }
    (buses, timestamp)
}
