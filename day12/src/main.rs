use core::ops::{Add, Mul, Sub};
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

const SHIP_START: Point = Point { x: 0, y: 0 };
const WAYPOINT_START: Point = Point { x: 10, y: -1 };

#[derive(Debug)]
struct Ship {
    position: Point,
    waypoint: Point,
    facing: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: SHIP_START,
            waypoint: WAYPOINT_START,
            facing: 0,
        }
    }

    fn act(&mut self, action: Action) {
        match action {
            Action::North(distance) => self.position = self.position + DIRECTIONS[3] * distance,
            Action::South(distance) => self.position = self.position + DIRECTIONS[1] * distance,
            Action::East(distance) => self.position = self.position + DIRECTIONS[0] * distance,
            Action::West(distance) => self.position = self.position + DIRECTIONS[2] * distance,
            Action::Left(degrees) => self.facing = (self.facing - degrees / 90 + 4) % 4,
            Action::Right(degrees) => self.facing = (self.facing + degrees / 90) % 4,
            Action::Forward(distance) => {
                self.position = self.position + DIRECTIONS[self.facing as usize] * distance
            }
        }
    }

    fn act_on_waypoint(&mut self, action: Action) {
        match action {
            Action::North(distance) => self.waypoint = self.waypoint + DIRECTIONS[3] * distance,
            Action::South(distance) => self.waypoint = self.waypoint + DIRECTIONS[1] * distance,
            Action::East(distance) => self.waypoint = self.waypoint + DIRECTIONS[0] * distance,
            Action::West(distance) => self.waypoint = self.waypoint + DIRECTIONS[2] * distance,
            Action::Forward(distance) => self.position = self.position + self.waypoint * distance,
            Action::Left(degrees) => {
                let mut d = degrees;
                while d > 0 {
                    let new_waypoint = Point {
                        x: self.waypoint.y,
                        y: 0 - self.waypoint.x,
                    };
                    self.waypoint = new_waypoint;
                    d -= 90;
                }
            }
            Action::Right(degrees) => {
                let mut d = degrees;
                while d > 0 {
                    let new_waypoint = Point {
                        x: 0 - self.waypoint.y,
                        y: self.waypoint.x,
                    };
                    self.waypoint = new_waypoint;
                    d -= 90;
                }
            }
        }
    }

    fn distance_from_start(&self) -> i32 {
        abs(self.position.x) + abs(self.position.y)
    }

    fn reset(&mut self) {
        self.position = SHIP_START;
        self.waypoint = WAYPOINT_START;
        self.facing = 0;
    }
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        0 - n
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn from(s: &str) -> Self {
        let (ch, digits) = s.split_at(1);
        let n = digits.parse::<i32>().unwrap();
        match ch {
            "N" => Self::North(n),
            "S" => Self::South(n),
            "E" => Self::East(n),
            "W" => Self::West(n),
            "L" => Self::Left(n),
            "R" => Self::Right(n),
            "F" => Self::Forward(n),
            _ => panic!("Invalid input"),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let actions: Vec<Action> = contents.lines().map(|s| Action::from(s)).collect();

    let mut ship = Ship::new();
    for action in &actions {
        ship.act(*action);
    }
    let part1 = ship.distance_from_start();
    assert_eq!(part1, 1603);
    println!("Part 1: {}", part1);

    ship.reset();
    for action in &actions {
        ship.act_on_waypoint(*action);
    }
    let part2 = ship.distance_from_start();
    assert_eq!(part2, 52866);
    println!("Part 2: {}", part2);
}
