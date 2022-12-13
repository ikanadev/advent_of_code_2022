use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn FnMut(u64) -> u64>,
    test_divisor: u64,
    throw_if_true: u64,
    throw_if_false: u64,
}
impl Monkey {
    fn print(&self) {
        print!("Monkey: ");
        print!(
            "] Div:{} t_true:{} t_false:{} items: [",
            self.test_divisor, self.throw_if_true, self.throw_if_false
        );
        for i in &self.items {
            print!("{},", i);
        }
        print!("]\n");
    }
}

fn parse_monkey_items(line: String) -> Vec<u64> {
    let colon_idx = line.find(':').unwrap();
    let mut substr: String = line[(colon_idx + 1)..].to_string();
    substr.retain(|c| !c.is_whitespace());
    let str_items = substr.split(',');
    let mut items = Vec::new();
    for item in str_items {
        let value: u64 = item.parse().unwrap();
        items.push(value);
    }
    items
}
fn get_last_digit(line: String) -> u64 {
    let items = line.split(' ');
    let divisor: u64 = items.last().unwrap().parse().unwrap();
    divisor
}
fn parse_operation(line: String) -> Box<dyn FnMut(u64) -> u64> {
    let equal_idx = line.find('=').unwrap();
    let substr = line[(equal_idx + 2)..].to_string();
    let str_items: Vec<&str> = substr.split(' ').collect();
    if str_items[0] == "old" && str_items[2] == "old" {
        if str_items[1] == "+" {
            return Box::new(|c| c + c);
        } else {
            return Box::new(|c| c * c);
            // return Box::new(|c| c);
        }
    } else {
        let mut number = str_items[0];
        if str_items[0] == "old" {
            number = str_items[2];
        }
        let number: u64 = number.parse().unwrap();
        if str_items[1] == "+" {
            return Box::new(move |c| c + number);
        } else {
            return Box::new(move |c| c * number);
            // return Box::new(move |c| c);
        }
    }
}
fn parse_monkey(lines: Vec<String>) -> Monkey {
    Monkey {
        items: parse_monkey_items(lines[1].clone()),
        op: parse_operation(lines[2].clone()),
        test_divisor: get_last_digit(lines[3].clone()),
        throw_if_true: get_last_digit(lines[4].clone()),
        throw_if_false: get_last_digit(lines[5].clone()),
    }
}

fn part_1(mut monkeys: Vec<Monkey>) {
    let rounds = 20;
    let mut monkey_items = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for j in 0..monkeys.len() {
            monkey_items[j] += monkeys[j].items.len();
            while monkeys[j].items.len() > 0 {
                let mut value = monkeys[j].items.remove(0);
                value = (monkeys[j].op)(value);
                value = value / 3;
                let target_idx;
                if value % monkeys[j].test_divisor == 0 {
                    target_idx = monkeys[j].throw_if_true;
                } else {
                    target_idx = monkeys[j].throw_if_false;
                }
                monkeys[target_idx as usize].items.push(value);
            }
        }
    }
    monkey_items.sort();
    let monkey_bussiness =
        monkey_items[monkey_items.len() - 1] * monkey_items[monkey_items.len() - 2];
    println!("Part 1: {}", monkey_bussiness);
}

fn part_2(mut monkeys: Vec<Monkey>) {
    // WIP
    let rounds = 12;
    let mut monkey_items = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for j in 0..monkeys.len() {
            monkey_items[j] += monkeys[j].items.len();
            while monkeys[j].items.len() > 0 {
                let mut value = monkeys[j].items.remove(0);
                value = (monkeys[j].op)(value);
                let target_idx;
                if value % monkeys[j].test_divisor == 0 {
                    target_idx = monkeys[j].throw_if_true;
                } else {
                    target_idx = monkeys[j].throw_if_false;
                }
                monkeys[target_idx as usize].items.push(value);
            }
        }
    }
    for m in monkeys {
        m.print();
    }
    println!("\n{:?}", monkey_items);
    /*
    monkey_items.sort();
    let monkey_bussiness =
        monkey_items[monkey_items.len() - 1] * monkey_items[monkey_items.len() - 2];
    println!("Part 1: {}", monkey_bussiness);
    */
}

fn main() {
    let file = File::open("input_small").unwrap();
    // let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut monkeys_part1 = Vec::new();
    let mut monkeys_part2 = Vec::new();

    let mut monkey_lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            monkeys_part1.push(parse_monkey(monkey_lines.clone()));
            monkeys_part2.push(parse_monkey(monkey_lines.clone()));
            monkey_lines = Vec::new();
            continue;
        }
        monkey_lines.push(line);
    }

    part_1(monkeys_part1);
    part_2(monkeys_part2);
}
