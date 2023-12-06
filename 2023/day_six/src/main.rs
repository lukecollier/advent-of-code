const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    one();
    two();
}

fn one() {
    let mut lines = PUZZLE_INPUT.lines();
    let times = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap());
    let mut acc: Option<usize> = None;
    for (time, target_distance) in times.zip(distances) {
        let mut valid_ways = 0;
        for hold_for in 1..time {
            let remaining_time = time - hold_for;
            let distance = hold_for * remaining_time;
            if distance > target_distance {
                valid_ways += 1;
            }
        }
        acc = acc.map_or_else(|| Some(valid_ways), |value| Some(value * valid_ways));
    }
    println!("part one: {}", &acc.unwrap())
}

fn two() {
    let mut lines = PUZZLE_INPUT.lines();
    let time = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let target_distance = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut acc: Option<usize> = None;
    let mut valid_ways = 0;
    for hold_for in 1..time {
        let remaining_time = time - hold_for;
        let distance = hold_for * remaining_time;
        if distance > target_distance {
            valid_ways += 1;
        }
    }
    acc = acc.map_or_else(|| Some(valid_ways), |value| Some(value * valid_ways));
    println!("part two: {}", &acc.unwrap())
}
