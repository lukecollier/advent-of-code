const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    one();
    two();
}
fn has_symbol_neighbour(lookup: &Vec<Vec<char>>, input_x: usize, input_y: usize) -> bool {
    fn is_symbol(char: &char) -> bool {
        char.is_ascii_punctuation() && char != &'.'
    }
    let x_max = lookup.first().unwrap().len();
    let y_max = lookup.len();
    let col = &lookup[(input_y.checked_sub(1).unwrap_or(0))..=(input_y + 1).min(y_max - 1)];
    let mut found = false;
    for row in col {
        let chars = &row[(input_x.checked_sub(1).unwrap_or(0))..=(input_x + 1).min(x_max - 1)];
        for char in chars {
            if is_symbol(char) {
                found = true;
            }
        }
    }
    found
}

fn one() {
    let mut total: usize = 0;

    let lookup: Vec<Vec<char>> = PUZZLE_INPUT
        .lines()
        .map(|line| line.chars().into_iter().collect())
        .collect();
    for (y, line) in PUZZLE_INPUT.lines().enumerate() {
        let mut buffer: String = String::new();
        let mut should_add_to_total: bool = false;
        for (x, char) in line.char_indices() {
            if char.is_ascii_digit() {
                buffer.push(char);
                if has_symbol_neighbour(&lookup, x, y) {
                    should_add_to_total = true;
                }
            } else if !buffer.is_empty() {
                if should_add_to_total {
                    total += buffer.parse::<usize>().unwrap();
                    should_add_to_total = false;
                }
                buffer.clear();
            }
        }
        if !buffer.is_empty() {
            if should_add_to_total {
                total += buffer.parse::<usize>().unwrap();
            }
        }
    }
    println!("problem one: {}", total)
}

fn check_row_for_numbers(row: &Vec<char>, x: usize) -> Vec<usize> {
    let mut buffer = String::new();
    let mut save = false;
    let mut strings: Vec<String> = Vec::new();
    for (idx, char) in row.iter().enumerate() {
        if char.is_ascii_digit() {
            buffer.push(*char);
            if (x - 1..=x + 1).contains(&idx) {
                save = true;
            }
        } else if !buffer.is_empty() {
            if save {
                strings.push(buffer.clone());
                save = false;
            }
            buffer.clear();
        }
        if row.get(idx + 1).is_none() && save && !buffer.is_empty() {
            strings.push(buffer.clone());
        }
    }
    strings
        .iter()
        .map(|string| string.parse::<usize>().unwrap())
        .collect()
}

// does a local search in the neighbourhood, if theres two nearby numbers returns them
fn two_neighbour_numbers(lookup: &Vec<Vec<char>>, x: usize, y: usize) -> Option<(usize, usize)> {
    let mut all: Vec<usize> = Vec::new();
    if y < lookup.len() {
        all.append(&mut check_row_for_numbers(&lookup[y + 1], x));
    }
    all.append(&mut check_row_for_numbers(&lookup[y], x));
    if y > 0 {
        all.append(&mut check_row_for_numbers(&lookup[y - 1], x));
    }
    if all.len() == 2 {
        Some((*all.first().unwrap(), *all.last().unwrap()))
    } else {
        None
    }
}

fn two() {
    let mut total: usize = 0;

    let lookup: Vec<Vec<char>> = PUZZLE_INPUT
        .lines()
        .map(|line| line.chars().into_iter().collect())
        .collect();
    for (y, line) in PUZZLE_INPUT.lines().enumerate() {
        for (x, char) in line.char_indices() {
            if char == '*' {
                if let Some((first, second)) = two_neighbour_numbers(&lookup, x, y) {
                    total += first * second;
                }
            }
        }
    }
    println!("problem two: {}", total)
}
