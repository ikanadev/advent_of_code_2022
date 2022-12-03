use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// problem statement: https://adventofcode.com/2022/day/1
fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let mut calories = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            calories.push(sum);
            sum = 0;
            continue;
        }
        let num = line.parse::<u32>().unwrap();
        sum += num;
    }
    calories.sort();
    calories.reverse();
    if calories.len() < 3 {
        return;
    }
    println!("Part1: {}", calories[0]);
    println!("Part1: {}", calories[0] + calories[1] + calories[2]);
}
