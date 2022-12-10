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
fn update_leader_position(leader: &mut Pos, dir: Dir) {
    match dir {
        Dir::Up => leader.y += 1,
        Dir::Down => leader.y -= 1,
        Dir::Left => leader.x -= 1,
        Dir::Right => leader.x += 1,
    }
}
fn check_and_update_pos(leader: &mut Pos, follower: &mut Pos) {
    if (leader.x - follower.x).abs() > 1 || (leader.y - follower.y).abs() > 1 {
        if leader.y > follower.y {
            follower.y += 1;
        }
        if leader.y < follower.y {
            follower.y -= 1;
        }
        if leader.x > follower.x {
            follower.x += 1;
        }
        if leader.x < follower.x {
            follower.x -= 1;
        }
    }
}
fn part1(dirs: Vec<Dir>) {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(tail.clone());
    for dir in dirs {
        update_leader_position(&mut head, dir);
        check_and_update_pos(&mut head, &mut tail);
        visited.insert(tail.clone());
    }
    println!("Part1: {}", visited.len());
}
fn part2(dirs: Vec<Dir>) {
    let mut pointers = Vec::from([
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
    ]);
    let last_idx = pointers.len() - 1;
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(pointers[last_idx].clone());
    for dir in dirs {
        update_leader_position(&mut pointers[0], dir);
        for i in 0..(pointers.len() - 1) {
            let mut leader = pointers[i].clone();
            let mut follower = pointers[i + 1].clone();
            check_and_update_pos(&mut leader, &mut follower);
            pointers[i] = leader;
            pointers[i + 1] = follower;
            // print!("{}:({},{}) ", i, pointers[i].x, pointers[i].y);
        }
        /*
        print!(
            "{}:({},{})\n",
            last_idx, pointers[last_idx].x, pointers[last_idx].y
        );
        */
        // draw_pointers(&pointers);
        visited.insert(pointers[last_idx].clone());
    }
    println!("Part2: {}", visited.len());
}
/* Used to visuallize
fn draw_pointers(pointers: &Vec<Pos>) {
    let size = 30;
    for i in (-10..size).rev() {
        for j in -10..size {
            let mut pointer: Option<usize> = None;
            for (nro, p) in pointers.iter().enumerate() {
                if p.x == j && p.y == i {
                    pointer = Some(nro);
                }
            }
            match pointer {
                Some(p) => print!("{}", p),
                None => print!("■"),
            }
        }
        print!("\n");
    }
}
*/

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
    part2(dirs.clone());
}
