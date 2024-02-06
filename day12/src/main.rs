use core::ops::{Add, Mul, Sub};
use std::fs;

#[derive(Copy, Clone)]
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

const START: Point = Point { x: 0, y: 0 };

struct Ship {
    position: Point,
    facing: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: START,
            facing: 0,
        }
    }

    fn act_on(&mut self, action: Action) {
        match action {
            Action::North(distance) => {
                self.position = self.position + DIRECTIONS[3] * distance;
            }
            Action::South(distance) => {
                self.position = self.position + DIRECTIONS[1] * distance;
            }
            Action::East(distance) => {
                self.position = self.position + DIRECTIONS[0] * distance;
            }
            Action::West(distance) => {
                self.position = self.position + DIRECTIONS[2] * distance;
            }
            Action::Left(degrees) => {
                self.facing = (self.facing - degrees / 90 + 4) % 4;
            }
            Action::Right(degrees) => {
                self.facing = (self.facing + degrees / 90) % 4;
            }
            Action::Forward(distance) => {
                self.position = self.position + DIRECTIONS[self.facing as usize] * distance;
            }
        }
    }

    fn distance_from_start(&self) -> i32 {
        abs(self.position.x - START.x) + abs(self.position.y - START.y)
    }
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        0 - n
    }
}

#[derive(Clone, Copy)]
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
    for action in actions {
        ship.act_on(action);
    }
    let part1 = ship.distance_from_start();
    println!("Part 1: {}", part1);
}
