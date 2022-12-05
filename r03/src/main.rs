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

// a = 97, A = 65
fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines() {
        // split line
        let line = line.unwrap();
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
