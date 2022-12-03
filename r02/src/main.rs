use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // PART 1
    // A: Rock, B: Paper, C: Scissors -> Opponent
    // X: Rock, Y: Paper, Z: Scissors -> Me
    // Rock: 1, Paper: 2, Scissors: 3
    // Win: 6, Draw: 3, Lose: 0
    // Opp->Me: Choice, Match = Total
    // A -> X: 1, 3 = 4
    // A -> Y: 2, 6 = 8
    // A -> Z: 3, 0 = 3
    // B -> X: 1, 0 = 1
    // B -> Y: 2, 3 = 5
    // B -> Z: 3, 6 = 9
    // C -> X: 1, 6 = 7
    // C -> Y: 2, 0 = 2
    // C -> Z: 3, 3 = 6
    let points_p1_map: HashMap<&str, u32> = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    // PART 2
    // A: Rock, B: Paper, C: Scissors -> Opponent
    // X: Lose, Y: Draw, Z: Win
    // Rock: 1, Paper: 2, Scissors: 3
    // Win: 6, Draw: 3, Lose: 0
    // Opp->MatchRes: Choice, Match = Total
    // A -> X: 3, 0 = 3
    // A -> Y: 1, 3 = 4
    // A -> Z: 2, 6 = 8
    // B -> X: 1, 0 = 1
    // B -> Y: 2, 3 = 5
    // B -> Z: 3, 6 = 9
    // C -> X: 2, 0 = 2
    // C -> Y: 3, 3 = 6
    // C -> Z: 1, 6 = 7
    let points_p2_map: HashMap<&str, u32> = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut points_p1 = Vec::new();
    let mut points_p2 = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let point_p1 = match points_p1_map.get(&*line) {
            Some(&num) => num,
            None => panic!("Cannot get points"),
        };
        points_p1.push(point_p1);
        let point_p2 = match points_p2_map.get(&*line) {
            Some(&num) => num,
            None => panic!("Cannot get points"),
        };
        points_p2.push(point_p2);
    }
    let total_points_p1: u32 = points_p1.iter().sum();
    let total_points_p2: u32 = points_p2.iter().sum();
    println!("Part1: {}", total_points_p1);
    println!("Part2: {}", total_points_p2);
}
