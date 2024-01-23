use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn get_tree_count(slope: &Point, trees: &HashSet<Point>, bounds: &Point) -> usize {
    let mut count: usize = 0;
    let mut position = Point { x: 0, y: 0 };
    while position.y <= bounds.y {
        if trees.contains(&position) {
            count += 1;
        }
        position.y += slope.y;
        position.x = (position.x + slope.x) % bounds.x;
    }
    count
}

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut trees = HashSet::<Point>::new();
    let mut x = 0;
    let mut y = 0;
    for ch in contents.chars() {
        match ch {
            '.' => x += 1,
            '#' => {
                trees.insert(Point { x: x, y: y });
                x += 1;
            }
            '\n' => {
                x = 0;
                y += 1;
            }
            _ => (),
        }
    }
    let slopes: Vec<Point> = vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];
    let bounds = Point { x: x, y: y };
    let tree_counts: Vec<usize> = slopes
        .iter()
        .map(|x| get_tree_count(&x, &trees, &bounds))
        .collect();
    let part1 = tree_counts[1];
    assert_eq!(part1, 278);
    println!("Part 1: {}", part1);
    let part2 = tree_counts.iter().product::<usize>();
    assert_eq!(part2, 9709761600);
    println!("Part 2: {}", part2);
}
