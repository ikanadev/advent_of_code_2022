use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Stack {
    items: Vec<char>,
}
impl Stack {
    fn new() -> Stack {
        Stack { items: Vec::new() }
    }
    fn add(&mut self, item: char) {
        self.items.push(item);
    }
    fn pop(&mut self) -> char {
        match self.items.pop() {
            Some(item) => item,
            None => panic!("Stack empty!"),
        }
    }
    fn check(&self) -> char {
        match self.items.last() {
            Some(item) => item.clone(),
            None => panic!("Stack empty!"),
        }
    }
}

fn parse_stacks(mut lines: Vec<String>) -> Vec<Stack> {
    lines.pop();
    lines.reverse();
    let qtty = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Stack> = Vec::new();
    for _ in 1..=qtty {
        stacks.push(Stack::new());
    }
    for line in lines {
        let mut idx = 1;
        for i in 0..qtty {
            let char = line.as_bytes()[idx] as char;
            if char != ' ' {
                stacks[i].add(char as char);
            }
            idx += 4;
        }
    }
    return stacks;
}

#[derive(Debug)]
struct Change {
    count: u32,
    from: u32,
    to: u32,
}

fn parse_changes(lines: Vec<String>) -> Vec<Change> {
    let mut changes: Vec<Change> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let count: u32 = parts[1].parse().unwrap();
        let from: u32 = parts[3].parse().unwrap();
        let to: u32 = parts[5].parse().unwrap();
        changes.push(Change {
            count,
            from: from - 1,
            to: to - 1,
        });
    }
    changes
}

fn part1(stacks: &mut Vec<Stack>, changes: &Vec<Change>) {
    for change in changes {
        for _ in 0..change.count {
            let char = stacks[change.from as usize].pop();
            stacks[change.to as usize].add(char);
        }
    }
    let mut str = String::new();
    for stack in stacks {
        str.push(stack.check());
    }
    println!("Part1: {}", str);
}

fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut change_lines: Vec<String> = Vec::new();
    let mut stack_lines: Vec<String> = Vec::new();
    let mut is_stack_line = true;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            is_stack_line = false;
            continue;
        }
        if is_stack_line {
            stack_lines.push(line);
        } else {
            change_lines.push(line);
        }
    }
    let mut stacks = parse_stacks(stack_lines);
    let changes = parse_changes(change_lines);

    part1(&mut stacks, &changes);
}
