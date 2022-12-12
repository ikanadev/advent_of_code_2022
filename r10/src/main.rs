use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
enum Inst {
    Noop,
    AddX(i32),
}
fn parse_ins(line: String) -> Inst {
    if line == "noop" {
        return Inst::Noop;
    }
    let line: Vec<&str> = line.split(" ").collect();
    let value: i32 = line[1].parse().unwrap();
    return Inst::AddX(value);
}

fn part1(insts: Vec<Inst>) -> [i32; 242] {
    let mut sections: [i32; 242] = [1; 242];
    let mut pointer = 0;
    for inst in insts {
        match inst {
            Inst::Noop => {
                pointer += 1;
            }
            Inst::AddX(n) => {
                let new_value = sections[pointer + 2] + n;
                for idx in (pointer + 2)..sections.len() {
                    sections[idx] = new_value;
                }
                pointer += 2;
            }
        }
    }
    let sections_to_check: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut total = 0;
    for section in sections_to_check {
        total += section * sections[section as usize - 1];
    }
    println!("Part1 :{}", total);
    sections
}
fn part2(sections: [i32; 242]) {
    let row_length = 40;
    let lit = '#';
    let blank = '.';
    let mut ouput = String::new();
    for i in 0..240 {
        let ctr_idx: i32 = i % row_length;
        if ctr_idx >= sections[i as usize] - 1 && ctr_idx <= sections[i as usize] + 1 {
            ouput.push(lit);
        } else {
            ouput.push(blank);
        }
    }
    print!("\n\nPart2:\n");
    for (i, c) in ouput.chars().enumerate() {
        if i as i32 % row_length == 0 {
            print!("\n");
        }
        print!("{}", c);
    }
    print!("\n");
}

fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut insts = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let inst = parse_ins(line);
        insts.push(inst);
    }
    let sections = part1(insts.clone());
    part2(sections);
}
