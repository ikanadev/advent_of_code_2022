use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOST: u32 = 0;

#[derive(PartialEq)]
enum Choice {
    Rock,     // 1
    Paper,    // 2
    Scissors, // 3
}
fn get_choice_points(choice: &Choice) -> u32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

struct Match {
    me: Choice,
    opponent: Choice,
}
impl Match {
    fn create(me: Choice, opponent: Choice) -> Match {
        Match { me, opponent }
    }
    fn get_points(self) -> u32 {
        let choice_points = get_choice_points(&self.me);
        let match_points = match self.me {
            Choice::Rock => match self.opponent {
                Choice::Rock => DRAW,
                Choice::Paper => LOST,
                Choice::Scissors => WIN,
            },
            Choice::Paper => match self.opponent {
                Choice::Rock => WIN,
                Choice::Paper => DRAW,
                Choice::Scissors => LOST,
            },
            Choice::Scissors => match self.opponent {
                Choice::Rock => LOST,
                Choice::Paper => WIN,
                Choice::Scissors => DRAW,
            },
        };
        return choice_points + match_points;
    }
}

fn parse_string_choice(choice: &str) -> Choice {
    // A: Rock, B: Paper, C: Scissors
    // X: Rock, Y: Paper, Z: Scissors
    match choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => {
            panic!("Invalid choice option")
        }
    }
}
fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut matches = Vec::new();
    let mut points = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let choices: Vec<&str> = line.split(" ").collect();
        matches.push(Match::create(
            parse_string_choice(choices[1]),
            parse_string_choice(choices[0]),
        ))
    }
    for m in matches {
        points.push(m.get_points())
    }
    let total_points: u32 = points.iter().sum();
    println!("Total points: {}", total_points)
}
