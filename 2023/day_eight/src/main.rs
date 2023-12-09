use num::Integer;
use std::{rc::Rc, time::Instant};

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    one();
    two();
}

fn one() {
    let mut lines = PUZZLE_INPUT.lines();
    let mut directions = std::iter::repeat(lines.next().unwrap().chars()).flatten();
    lines.next();
    let new_lines = std::iter::repeat(lines).flatten();
    let mut searching_for = "AAA";
    let mut steps = 0;
    for line in new_lines {
        let instant = Instant::now();
        let mut splits = line.split_ascii_whitespace();
        let current = splits.next().unwrap();
        splits.next();
        let left_raw = splits.next().unwrap();
        let right_raw = splits.next().unwrap();
        let left = &left_raw[1..left_raw.len() - 1];
        let right = &right_raw[0..left_raw.len() - 2];
        if searching_for == current {
            let direction = directions.next().unwrap();
            searching_for = match direction {
                'R' => right,
                'L' => left,
                _ => todo!(),
            };
            steps += 1;
        }
        println!("took {:?}", instant.elapsed());
        if searching_for == "ZZZ" {
            break;
        }
    }

    println!("part one: {}", &steps);
}

fn two() {
    use std::collections::HashMap;

    let mut lines = PUZZLE_INPUT.lines();
    let directions = std::iter::repeat(lines.next().unwrap().char_indices()).flatten();
    lines.next();
    let mut currently_on: Vec<&str> = Vec::new();
    let mut world: HashMap<&str, (&str, &str)> = HashMap::new();
    // find all our starting positions and build the world
    for line in lines {
        let mut splits = line.split_ascii_whitespace();
        let current = splits.next().unwrap();
        let current_end = &current.chars().last().unwrap();
        splits.next();
        let left_raw = splits.next().unwrap();
        let right_raw = splits.next().unwrap();
        let left = &left_raw[1..left_raw.len() - 1];
        let right = &right_raw[0..left_raw.len() - 2];
        if current_end == &'A' {
            currently_on.push(current);
        }
        world.insert(current, (left, right));
    }

    let mut lcm: Option<usize> = None;
    for idx in 0..currently_on.len() {
        let mut steps = Vec::new();
        let mut directions = directions.clone();
        loop {
            let cur = currently_on[idx];
            let (left, right) = world.get(currently_on[idx]).unwrap();
            let (direction_id, direction) = directions.next().unwrap();
            if direction == 'L' {
                currently_on[idx] = left;
            } else if direction == 'R' {
                currently_on[idx] = right;
            }
            let candidate = (cur, direction_id);
            if steps.contains(&candidate) {
                // we've found the loop
                println!("loop encountered {:?} steps", steps.len());
                break;
            } else {
                steps.push(candidate);
            }
        }
        for (idx, (cur, _)) in steps.iter().enumerate() {
            if &cur[cur.len() - 1..cur.len()] == "Z" {
                lcm = Some(lcm.get_or_insert(idx).lcm(&idx));
            }
        }
    }
    println!("part two: {}", lcm.unwrap());
}
