use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Cube(i32, i32, i32);

impl Add for Cube {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct HyperCube(i32, i32, i32, i32);

impl Add for HyperCube {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

fn _print_state(state: &HashSet<Cube>) {
    let (min_x, min_y, min_z, max_x, max_y, max_z) = bounds(state);
    for z in min_z..=max_z {
        println!("z={}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if state.contains(&Cube(x, y, z)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
    println!();
}

fn bounds(state: &HashSet<Cube>) -> (i32, i32, i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for cube in state {
        let (x, y, z) = (cube.0, cube.1, cube.2);
        min_x = min(x, min_x);
        min_y = min(y, min_y);
        min_z = min(z, min_z);
        max_x = max(x, max_x);
        max_y = max(y, max_y);
        max_z = max(z, max_z);
    }
    (min_x, min_y, min_z, max_x, max_y, max_z)
}

fn perform_round(state: &HashSet<Cube>, deltas: &Vec<Cube>) -> HashSet<Cube> {
    let (min_x, min_y, min_z, max_x, max_y, max_z) = bounds(state);
    let mut new_state: HashSet<Cube> = HashSet::new();
    for z in min_z - 1..=max_z + 1 {
        for y in min_y - 1..=max_y + 1 {
            'outer: for x in min_x - 1..=max_x + 1 {
                let cube = Cube(x, y, z);
                let mut active_count = 0;
                for delta in deltas {
                    let neighbor = cube + *delta;
                    if state.contains(&neighbor) {
                        active_count += 1;
                    }
                    if active_count > 3 {
                        continue 'outer;
                    }
                }
                if active_count == 3 || (state.contains(&cube) && active_count == 2) {
                    new_state.insert(cube);
                }
            }
        }
    }
    new_state
}

fn main() {
    let nums: [i32; 8] = [1, 1, 1, 0, 0, -1, -1, -1];
    let deltas: Vec<Cube> = nums
        .iter()
        .permutations(3)
        .unique()
        .map(|p| Cube(*p[0], *p[1], *p[2]))
        .collect();

    let contents: String = fs::read_to_string("data.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    let z = 0;
    let mut state: HashSet<Cube> = HashSet::new();
    for ch in contents.chars() {
        if ch == '\n' {
            x = 0;
            y += 1;
            continue;
        }
        if ch == '#' {
            state.insert(Cube(x, y, z));
        }
        x += 1;
    }
    for _ in 0..6 {
        state = perform_round(&state, &deltas);
    }
    let part1 = state.len();
    assert_eq!(part1, 301);
    println!("Part 1: {}", part1);

    let nums: [i32; 11] = [1, 1, 1, 1, 0, 0, 0, -1, -1, -1, -1];
    let deltas: Vec<HyperCube> = nums
        .iter()
        .permutations(4)
        .unique()
        .map(|p| HyperCube(*p[0], *p[1], *p[2], *p[3]))
        .collect();
    println!("{:?}", deltas);
}
