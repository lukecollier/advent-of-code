const PUZZLE_INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("puzzle one: {}", one());
    println!("puzzle two: {}", two());
}

fn one() -> isize {
    let mut total = 0;
    for line in PUZZLE_INPUT.lines() {
        let mut numbers: Vec<Vec<_>> = vec![line
            .split_ascii_whitespace()
            .map(|number| number.parse::<isize>().unwrap())
            .collect::<Vec<_>>()];
        let mut idx = 0;
        loop {
            let current_numbers = numbers.get(idx).unwrap();
            let number_window = current_numbers.windows(2);
            let mut differences = Vec::with_capacity(current_numbers.len() - 1);
            for window in number_window {
                let first = window.first().unwrap();
                let last = window.last().unwrap();
                let diff = last - first;
                differences.push(diff.try_into().unwrap());
            }
            if differences.iter().all(|is_zero| is_zero == &0) {
                break;
            }
            numbers.push(differences);
            idx += 1;
        }
        let total_len = numbers.len();
        for id in 1..total_len {
            let change_by = numbers.get(total_len - id).unwrap().last().unwrap().clone();
            let current = numbers.get_mut(total_len - id - 1).unwrap();
            let last = current.last().unwrap();
            current.push(last + change_by);
        }
        total += numbers.first().unwrap().last().unwrap();
    }

    total
}

fn two() -> isize {
    let mut total = 0;
    for line in PUZZLE_INPUT.lines() {
        let mut numbers: Vec<Vec<_>> = vec![line
            .split_ascii_whitespace()
            .map(|number| number.parse::<isize>().unwrap())
            .rev()
            .collect::<Vec<_>>()];
        let mut idx = 0;
        loop {
            let current_numbers = numbers.get(idx).unwrap();
            let number_window = current_numbers.windows(2);
            let mut differences = Vec::with_capacity(current_numbers.len() - 1);
            for window in number_window {
                let first = window.first().unwrap();
                let last = window.last().unwrap();
                let diff = last - first;
                differences.push(diff.try_into().unwrap());
            }
            if differences.iter().all(|is_zero| is_zero == &0) {
                break;
            }
            numbers.push(differences);
            idx += 1;
        }
        let total_len = numbers.len();
        for id in 1..total_len {
            let change_by = numbers.get(total_len - id).unwrap().last().unwrap().clone();
            let current = numbers.get_mut(total_len - id - 1).unwrap();
            let last = current.last().unwrap();
            current.push(last + change_by);
        }
        total += numbers.first().unwrap().last().unwrap();
    }

    total
}
