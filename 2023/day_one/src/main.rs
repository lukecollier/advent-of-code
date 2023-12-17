fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let puzzle_input = include_str!("./input.txt").lines();
    let mut total = 0;
    for line in puzzle_input {
        let ascii_digits: Vec<_> = line
            .chars()
            .into_iter()
            .filter_map(|char| {
                if char.is_ascii_digit() {
                    Some(char)
                } else {
                    None
                }
            })
            .collect();
        let mut first = ascii_digits.first().unwrap().to_string();
        let last = ascii_digits.last().unwrap();
        first.push(*last);
        let number = first.parse::<i32>().unwrap();
        total += number;
    }
    println!("part one: {}", total);
}

fn to_digit(str: &str) -> Option<char> {
    match str {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn part_two() {
    let puzzle_input = include_str!("./input.txt").lines();
    let mut total = 0;

    for line in puzzle_input {
        let mut new_line: String = String::new();
        for (idx, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                new_line.push(char);
            }
            if char.is_ascii_alphabetic() {
                let three_letters = &line[idx..line.len().min(idx + 3)];
                let four_letters = &line[idx..line.len().min(idx + 4)];
                let five_letters = &line[idx..line.len().min(idx + 5)];
                if to_digit(three_letters).is_some() {
                    new_line.push(to_digit(three_letters).unwrap());
                } else if to_digit(four_letters).is_some() {
                    new_line.push(to_digit(four_letters).unwrap());
                } else if to_digit(five_letters).is_some() {
                    new_line.push(to_digit(five_letters).unwrap());
                }
            }
        }
        let ascii_digits: Vec<_> = new_line
            .chars()
            .into_iter()
            .filter_map(|char| {
                if char.is_ascii_digit() {
                    Some(char)
                } else {
                    None
                }
            })
            .collect();
        let mut first = ascii_digits.first().unwrap().to_string();
        let last = ascii_digits.last().unwrap();
        first.push(*last);
        let number = first.parse::<i32>().unwrap();
        total += number;
    }
    println!("part two: {}", total);
}
