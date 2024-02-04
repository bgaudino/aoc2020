use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Seat {
    x: i32,
    y: i32,
}

impl Seat {
    fn get_neighbors(&self) -> [Self; 8] {
        [
            Seat {
                x: self.x - 1,
                y: self.y - 1,
            },
            Seat {
                x: self.x,
                y: self.y - 1,
            },
            Seat {
                x: self.x + 1,
                y: self.y - 1,
            },
            Seat {
                x: self.x - 1,
                y: self.y,
            },
            Seat {
                x: self.x + 1,
                y: self.y,
            },
            Seat {
                x: self.x - 1,
                y: self.y + 1,
            },
            Seat {
                x: self.x,
                y: self.y + 1,
            },
            Seat {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

struct SeatingChart {
    seats: HashMap<Seat, bool>,
}

impl SeatingChart {
    fn from(s: String) -> Self {
        let mut seats: HashMap<Seat, bool> = HashMap::new();
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
            }
        }
        Self { seats }
    }

    fn rearrange(&mut self) -> bool {
        let mut changed = false;
        let mut new_seats: HashMap<Seat, bool> = HashMap::new();
        for (seat, taken) in &self.seats {
            let seat2 = Seat {
                x: seat.x,
                y: seat.y,
            };
            let neighbors = seat.get_neighbors();
            if !taken {
                let mut occupied_seats = false;
                for n in &neighbors {
                    match self.seats.get(n) {
                        Some(t) => {
                            if *t {
                                occupied_seats = true;
                                break;
                            }
                        }
                        None => (),
                    }
                }
                if occupied_seats {
                    new_seats.insert(seat2, false);
                } else {
                    new_seats.insert(seat2, true);
                    changed = true;
                }
            } else {
                let mut num_occupied = 0;
                for n in &neighbors {
                    match self.seats.get(n) {
                        Some(t) => {
                            if *t {
                                num_occupied += 1;
                                if num_occupied >= 4 {
                                    break;
                                }
                            }
                        }
                        None => (),
                    }
                }
                if num_occupied >= 4 {
                    new_seats.insert(seat2, false);
                    changed = true;
                } else {
                    new_seats.insert(seat2, true);
                }
            }
        }
        self.seats = new_seats;
        changed
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
}

fn main() {
    let content = fs::read_to_string("data.txt").unwrap();
    let mut chart = SeatingChart::from(content);
    loop {
        if !chart.rearrange() {
            break;
        }
    }
    println!("Part 1: {}", chart.num_occupied());
}
