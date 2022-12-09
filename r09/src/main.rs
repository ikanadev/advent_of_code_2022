use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
fn parse_line_to_dirs(line: String) -> Vec<Dir> {
    let line: Vec<&str> = line.split(" ").collect();
    let mut dirs = Vec::new();
    let dir = match line[0] {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        &_ => panic!("Ivalid dir input"),
    };
    let qtty: i32 = line[1].parse().unwrap();
    for _ in 1..=qtty {
        dirs.push(dir);
    }
    dirs
}
fn part1(dirs: Vec<Dir>) {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(tail.clone());
    for dir in dirs {
        match dir {
            Dir::Up => head.y += 1,
            Dir::Down => head.y -= 1,
            Dir::Left => head.x -= 1,
            Dir::Right => head.x += 1,
        }
        if (head.x - tail.x).abs() > 1 {
            match dir {
                Dir::Left => tail.x -= 1,
                Dir::Right => tail.x += 1,
                Dir::Up | Dir::Down => {}
            }
            if head.y != tail.y {
                tail.y = head.y;
            }
        }
        if (head.y - tail.y).abs() > 1 {
            match dir {
                Dir::Up => tail.y += 1,
                Dir::Down => tail.y -= 1,
                Dir::Left | Dir::Right => {}
            }
            if head.x != tail.x {
                tail.x = head.x;
            }
        }
        visited.insert(tail.clone());
    }
    println!("Part1: {}", visited.len());
}

fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut dirs: Vec<Dir> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        dirs.append(&mut parse_line_to_dirs(line));
    }
    part1(dirs.clone());
}
