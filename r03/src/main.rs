use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn get_numeric_value(char: char) -> u32 {
    let value = char as u32;
    if value >= 97 && value <= 122 {
        return char as u32 - 96;
    }
    if value >= 65 && value <= 90 {
        return char as u32 - 38;
    }
    0
}

fn part1(lines: Vec<String>) {
    let mut sum: u32 = 0;
    for line in lines {
        // split line
        let half = &line[0..line.len() / 2];
        let rest = &line[line.len() / 2..];
        // save chars count to hashmap
        let mut chars: HashMap<char, u32> = HashMap::new();
        for char in half.chars() {
            chars
                .entry(char)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        // get douplicated values
        let mut doubles: HashSet<char> = HashSet::new();
        for char in rest.chars() {
            match chars.get(&char) {
                Some(..) => {
                    doubles.insert(char);
                }
                None => {
                    continue;
                }
            };
        }
        // sum numeric values
        for char in doubles.clone() {
            sum += get_numeric_value(char);
        }
    }
    println!("Part1: {}", sum);
}

fn part2(lines: Vec<String>) {
    let mut counter = 1;
    let mut sum: u32 = 0;
    let mut chars: HashMap<char, u32> = HashMap::new();
    for line in lines {
        let mut chars_set: HashSet<char> = HashSet::new();
        for char in line.chars() {
            chars_set.insert(char);
        }
        for char in chars_set {
            chars.entry(char).and_modify(|c| *c += 1).or_insert(1);
        }
        if counter == 3 {
            counter = 0;
            for (char, count) in &chars {
                if count == &3 {
                    sum += get_numeric_value(*char);
                }
            }
            chars = HashMap::new();
        }
        counter += 1;
    }
    println!("Part2: {}", sum);
}

// a = 97, A = 65
fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = String::from(line);
        lines.push(line.clone());
    }
    part1(lines.clone());
    part2(lines.clone());
}
