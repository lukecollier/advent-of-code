const PUZZLE_INPUT: &str = include_str!("./input.txt");
fn main() {
    one();
    two();
}
fn one() {
    let mut total: usize = 0;
    for line in PUZZLE_INPUT.lines() {
        if let Some((_, rest)) = line.split_once(":") {
            if let Some((left, right)) = rest.split_once("|") {
                let winners: Vec<_> = left
                    .split_ascii_whitespace()
                    .map(|digits| digits.parse::<usize>().unwrap())
                    .collect();
                let numbers = right.split_ascii_whitespace().filter_map(|digits| {
                    let number = digits.parse::<usize>().unwrap();
                    if winners.contains(&number) {
                        Some(number)
                    } else {
                        None
                    }
                });
                if &numbers.clone().count() > &0 {
                    let mut current = 1;
                    for _ in numbers.skip(1) {
                        current = current * 2;
                    }
                    total += current;
                }
            }
        }
    }
    println!("problem one {}", total);
}

fn two() {
    let mut scratch_cards_count: usize = PUZZLE_INPUT.lines().count();
    let mut cards_won: Vec<usize> = Vec::new();
    for line in PUZZLE_INPUT.lines() {
        if let Some((card, rest)) = line.split_once(":") {
            let card_number = card
                .chars()
                .skip_while(|char| !char.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            if let Some((left, right)) = rest.split_once("|") {
                let winners: Vec<_> = left
                    .split_ascii_whitespace()
                    .map(|digits| digits.parse::<usize>().unwrap())
                    .collect();
                let won = right
                    .split_ascii_whitespace()
                    .filter_map(|digits| {
                        let number = digits.parse::<usize>().unwrap();
                        if winners.contains(&number) {
                            Some(number)
                        } else {
                            None
                        }
                    })
                    .count();
                let repeat_by = cards_won
                    .iter()
                    .filter(|card| card == &&card_number)
                    .count();
                for card in ((card_number + 1)..=card_number + won)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .repeat(repeat_by + 1)
                {
                    scratch_cards_count += 1;
                    cards_won.push(card)
                }
            }
        }
    }
    println!("problem two {}", scratch_cards_count);
}
