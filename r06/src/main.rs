use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn are_unique(items: Vec<u8>) -> bool {
    for i in 0..(items.len() - 1) {
        for j in (i + 1)..items.len() {
            if items[i] == items[j] {
                return false;
            }
        }
    }
    return true;
}

fn part1(line: String) {
    let start_idx = 3;
    let line_bytes = line.as_bytes();
    for idx in start_idx..line.len() {
        let first_four: Vec<u8> = Vec::from([
            line_bytes[idx],
            line_bytes[idx - 1],
            line_bytes[idx - 2],
            line_bytes[idx - 3],
        ]);
        if are_unique(first_four) {
            println!("Part1: {}", idx + 1);
            break;
        }
    }
}
fn part2(line: String) {
    let start_idx = 13;
    let line_bytes = line.as_bytes();
    for idx in start_idx..line.len() {
        let first_fourteen: Vec<u8> = Vec::from([
            line_bytes[idx],
            line_bytes[idx - 1],
            line_bytes[idx - 2],
            line_bytes[idx - 3],
            line_bytes[idx - 4],
            line_bytes[idx - 5],
            line_bytes[idx - 6],
            line_bytes[idx - 7],
            line_bytes[idx - 8],
            line_bytes[idx - 9],
            line_bytes[idx - 10],
            line_bytes[idx - 11],
            line_bytes[idx - 12],
            line_bytes[idx - 13],
        ]);
        if are_unique(first_fourteen) {
            println!("Part2: {}", idx + 1);
            break;
        }
    }
}

fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        part1(line.clone());
        part2(line.clone());
    }
}
