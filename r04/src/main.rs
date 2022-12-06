use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Section {
    from: u32,
    to: u32,
}

fn are_sections_contained(s1: Section, s2: Section) -> bool {
    if s1.from <= s2.from && s1.to >= s2.to {
        return true;
    }
    if s2.from <= s1.from && s2.to >= s1.to {
        return true;
    }
    false
}

fn are_sections_overlaped(s1: Section, s2: Section) -> bool {
    if s1.from < s2.from && s1.to < s2.from {
        return false;
    }
    if s2.from < s1.from && s2.to < s1.from {
        return false;
    }
    true
}

fn parse_section(str: String) -> Section {
    let numbers: Vec<&str> = str.split('-').collect();
    let from: u32 = numbers[0].parse().unwrap();
    let to: u32 = numbers[1].parse().unwrap();
    return Section { from, to };
}

fn parse_line(str: String) -> (String, String) {
    let numbers: Vec<&str> = str.split(',').collect();
    return (String::from(numbers[0]), String::from(numbers[1]));
}

fn part_1(lines: Vec<String>) {
    let mut count: u32 = 0;
    for line in lines {
        let (one, two) = parse_line(line);
        let section_one = parse_section(one);
        let section_two = parse_section(two);
        if are_sections_contained(section_one, section_two) {
            count += 1;
        }
    }
    println!("Part1: {}", count);
}
fn part_2(lines: Vec<String>) {
    let mut count: u32 = 0;
    for line in lines {
        let (one, two) = parse_line(line);
        let section_one = parse_section(one);
        let section_two = parse_section(two);
        if are_sections_overlaped(section_one, section_two) {
            count += 1;
        }
    }
    println!("Part2: {}", count);
}

fn main() {
    let file = File::open("input_small").unwrap();
    // let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }
    part_1(lines.clone());
    part_2(lines.clone());
}
