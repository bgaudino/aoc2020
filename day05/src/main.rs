use std::fs;

const NUM_ROWS: u32 = 128;
const NUM_COLUMNS: u32 = 8;

fn partition(instructions: &str, size: u32) -> u32 {
    let mut start: u32 = 0;
    let mut end: u32 = size - 1;
    for ch in instructions.chars() {
        let middle = (end - start) / 2 + start;
        if ch == 'F' || ch == 'L' {
            end = middle;
        } else {
            start = middle + 1;
        }
    }
    start
}

fn find_my_seat(mut taken_seats: Vec<u32>) -> Option<u32> {
    taken_seats.sort();
    let mut i = taken_seats[0];
    for seat in taken_seats {
        if seat != i {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn main() {
    let mut taken_seats: Vec<u32> = Vec::new();
    let mut max_seat_id: u32 = 0;
    for pass in fs::read_to_string("data.txt").unwrap().lines() {
        let row_instructions = &pass[..7];
        let column_instructions = &pass[7..];
        let row = partition(row_instructions, NUM_ROWS);
        let column = partition(column_instructions, NUM_COLUMNS);
        let seat_id = row * 8 + column;
        taken_seats.push(seat_id);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    assert_eq!(max_seat_id, 935);
    println!("Part 1: {}", max_seat_id);
    let my_seat = find_my_seat(taken_seats).unwrap();
    assert_eq!(my_seat, 743);
    println!("Part 2: {}", my_seat);
}
