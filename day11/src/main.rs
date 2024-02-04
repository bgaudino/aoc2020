use std::cmp::max;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Seat {
    x: i32,
    y: i32,
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct SeatingChart {
    seats: HashMap<Seat, bool>,
    max_x: i32,
    max_y: i32,
}

impl SeatingChart {
    fn from(s: String) -> Self {
        let mut seats: HashMap<Seat, bool> = HashMap::new();
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;
        for (y, row) in s.lines().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                let seat = Seat {
                    x: x as i32,
                    y: y as i32,
                };
                if ch == 'L' {
                    seats.insert(seat, false);
                } else if ch == '#' {
                    seats.insert(seat, true);
                }
                max_x = max(x as i32, max_x);
            }
            max_y = max(y as i32, max_y);
        }
        Self {
            seats,
            max_x,
            max_y,
        }
    }

    fn rearrange(&mut self, part2: bool) -> bool {
        let mut changed = false;
        let mut new_seats: HashMap<Seat, bool> = HashMap::new();
        for (seat, taken) in &self.seats {
            let new_seat = seat.clone();
            let mut new_taken = *taken;
            let num = if !*taken {
                1
            } else if part2 {
                4
            } else {
                5
            };
            if self.should_flip(seat, num, part2) {
                new_taken = !new_taken;
                changed = true;
            }
            new_seats.insert(new_seat, new_taken);
        }
        self.seats = new_seats;
        changed
    }

    fn should_flip(&self, seat: &Seat, num: i32, part2: bool) -> bool {
        let is_taken = self.is_taken(seat);
        let mut count = 0;
        for direction in &DIRECTIONS {
            let neighbor = self.get_neighbor(seat, *direction, part2);
            let neighbor_taken = if neighbor.is_none() {
                false
            } else {
                self.is_taken(&neighbor.unwrap())
            };
            if neighbor_taken != is_taken {
                count += 1;
            }
            if count >= num {
                return false;
            }
        }
        true
    }

    fn is_taken(&self, seat: &Seat) -> bool {
        match self.seats.get(seat) {
            Some(taken) => *taken,
            None => false,
        }
    }

    fn get_neighbor(&self, seat: &Seat, direction: (i32, i32), part2: bool) -> Option<Seat> {
        if part2 {
            self.first_neighbor(seat, direction)
        } else {
            Some(Seat {
                x: seat.x + direction.0,
                y: seat.y + direction.1,
            })
        }
    }

    fn first_neighbor(&self, seat: &Seat, direction: (i32, i32)) -> Option<Seat> {
        let (dx, dy) = direction;
        let mut neighbor = seat.clone();
        while neighbor.x >= 0
            && neighbor.x <= self.max_x
            && neighbor.y >= 0
            && neighbor.y <= self.max_y
        {
            neighbor.x += dx;
            neighbor.y += dy;
            let taken = self.seats.get(&neighbor);
            if taken.is_some() {
                return Some(neighbor);
            }
        }
        None
    }

    fn num_occupied(self) -> i32 {
        let mut count = 0;
        for (_, taken) in &self.seats {
            if *taken {
                count += 1;
            }
        }
        count
    }

    fn _print(&self) {
        for i in 0..=self.max_y {
            for j in 0..=self.max_x {
                match self.seats.get(&Seat { x: j, y: i }) {
                    Some(taken) => {
                        if *taken {
                            print!("#");
                        } else {
                            print!("L");
                        }
                    }
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let mut chart = SeatingChart::from(content);
    let mut chart2 = SeatingChart {
        seats: chart.seats.clone(),
        max_x: chart.max_x,
        max_y: chart.max_y,
    };
    loop {
        if !chart.rearrange(false) {
            break;
        }
    }
    let part1 = chart.num_occupied();
    assert_eq!(part1, 2270);
    println!("Part 1: {}", part1); 

    loop {
        if !chart2.rearrange(true) {
            break;
        }
    }
    let part2 = chart2.num_occupied();
    assert_eq!(part2, 2042);
    println!("Part 2: {}", part2); 
}
