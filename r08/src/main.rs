use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_visible(row_idx: usize, col_idx: usize, matrix: &Vec<Vec<i32>>) -> bool {
    let size = matrix.len();
    let value = matrix[row_idx][col_idx];
    let mut visible_directions = vec![true, true, true, true];
    // left check
    for i in 0..col_idx {
        if matrix[row_idx][i] >= value {
            visible_directions[0] = false;
            break;
        }
    }
    if visible_directions[0] {
        return visible_directions[0];
    }
    // right check
    for i in (col_idx + 1)..size {
        if matrix[row_idx][i] >= value {
            visible_directions[1] = false;
            break;
        }
    }
    if visible_directions[1] {
        return visible_directions[1];
    }
    // up check
    for i in 0..row_idx {
        if matrix[i][col_idx] >= value {
            visible_directions[2] = false;
            break;
        }
    }
    if visible_directions[2] {
        return visible_directions[2];
    }
    // down check
    for i in (row_idx + 1)..size {
        if matrix[i][col_idx] >= value {
            visible_directions[3] = false;
            break;
        }
    }
    if visible_directions[3] {
        return visible_directions[3];
    }

    false
}

fn part1(matrix: &Vec<Vec<i32>>) {
    let size = matrix.len();
    let mut visible_counter = 0;
    for row_idx in 1..(size - 1) {
        for col_idx in 1..(size - 1) {
            if is_visible(row_idx, col_idx, matrix) {
                visible_counter += 1;
            }
        }
    }
    let visible_edges = size * 2 + 2 * (size - 2);
    println!("Part1: {}", visible_counter + visible_edges);
}

fn get_scenic_score(row_idx: usize, col_idx: usize, matrix: &Vec<Vec<i32>>) -> i32 {
    let size = matrix.len();
    let value = matrix[row_idx][col_idx];
    let mut scores = vec![0, 0, 0, 0];
    // left check
    for i in (0..col_idx).rev() {
        scores[0] += 1;
        if matrix[row_idx][i] >= value {
            break;
        }
    }
    // right check
    for i in (col_idx + 1)..size {
        scores[1] += 1;
        if matrix[row_idx][i] >= value {
            break;
        }
    }
    // up check
    for i in (0..row_idx).rev() {
        scores[2] += 1;
        if matrix[i][col_idx] >= value {
            break;
        }
    }
    // down check
    for i in (row_idx + 1)..size {
        scores[3] += 1;
        if matrix[i][col_idx] >= value {
            break;
        }
    }
    return scores.iter().product();
}

fn part2(matrix: &Vec<Vec<i32>>) {
    let size = matrix.len();
    let mut max_score = 0;
    for row_idx in 1..(size - 1) {
        for col_idx in 1..(size - 1) {
            let score = get_scenic_score(row_idx, col_idx, matrix);
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("Part2: {}", max_score);
}

fn main() {
    // let file = File::open("input_small").unwrap();
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for char in line.chars() {
            let number: i32 = String::from(char).parse().unwrap();
            row.push(number);
        }
        matrix.push(row);
    }
    part1(&matrix);
    part2(&matrix);
}
