const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    one();
    two();
}

fn one() {
    fn letter_to_strength(char: char) -> u32 {
        match char {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => panic!("OH NO"),
        }
    }

    fn hand_strength(cards: &str) -> u32 {
        let mut chars = cards.chars().collect::<Vec<_>>();
        chars.sort();
        let mut seen: Vec<char> = Vec::new();
        let mut score: usize = 0;
        for char in chars.into_iter() {
            let seen_amount = &seen
                .clone()
                .into_iter()
                .filter(|look| look == &char)
                .count();
            score += seen_amount;
            seen.push(char);
        }
        score.try_into().unwrap()
    }
    fn is_highest(cards: &str, other: &str) -> bool {
        for (left, right) in cards.chars().zip(other.chars()) {
            let strength_left = letter_to_strength(left);
            let strength_right = letter_to_strength(right);
            if strength_left > strength_right {
                return true;
            } else if strength_right > strength_left {
                return false;
            }
        }
        // this technically means a draw :shrug:
        return false;
    }
    let mut ranks: Vec<(&str, usize)> = Vec::new();
    for line in PUZZLE_INPUT.lines() {
        if let Some((hand, bid)) = line.split_once(" ") {
            ranks.push((hand, bid.parse::<usize>().unwrap()));
        }
    }
    ranks.sort_by(|(hand_one, _), (hand_two, _)| {
        is_highest(hand_one, hand_two).cmp(&is_highest(hand_two, hand_one))
    });
    ranks.sort_by(|(hand_one, _), (hand_two, _)| {
        hand_strength(&hand_one).cmp(&hand_strength(&hand_two))
    });

    let result = &ranks
        .iter()
        .enumerate()
        .fold(0, |acc, (rank_minus_one, (_, bid))| {
            acc + ((rank_minus_one + 1) * bid)
        });
    println!("one: {}", result);
}

fn two() {
    fn letter_to_strength(char: char) -> u32 {
        match char {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            'J' => 0,
            _ => panic!("OH NO"),
        }
    }
    use std::collections::BTreeMap;
    fn hand_strength(cards: &str) -> u32 {
        let mut counts = BTreeMap::new();
        for card in cards.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }
        let mut counts_as_vec = counts.into_iter().collect::<Vec<_>>();
        counts_as_vec.sort_by(|(_, cmp1), (_, cmp2)| cmp1.cmp(&cmp2));

        let first = counts_as_vec.first().unwrap();
        let mut replace_j_with = first.0;
        let mut chars = cards
            .chars()
            .map(|char| if char == 'J' { replace_j_with } else { char })
            .collect::<Vec<_>>();
        chars.sort();
        let mut seen: Vec<char> = Vec::new();
        let mut score: usize = 0;
        for char in chars.into_iter() {
            let seen_amount = &seen
                .clone()
                .into_iter()
                .filter(|look| look == &char)
                .count();
            score += seen_amount;
            seen.push(char);
        }
        score.try_into().unwrap()
    }
    fn is_highest(cards: &str, other: &str) -> bool {
        for (left, right) in cards.chars().zip(other.chars()) {
            let strength_left = letter_to_strength(left);
            let strength_right = letter_to_strength(right);
            if strength_left > strength_right {
                return true;
            } else if strength_right > strength_left {
                return false;
            }
        }
        // this technically means a draw :shrug:
        return false;
    }
    let mut ranks: Vec<(&str, usize)> = Vec::new();
    for line in PUZZLE_INPUT.lines() {
        if let Some((hand, bid)) = line.split_once(" ") {
            ranks.push((hand, bid.parse::<usize>().unwrap()));
        }
    }
    ranks.sort_by(|(hand_one, _), (hand_two, _)| {
        is_highest(hand_one, hand_two).cmp(&is_highest(hand_two, hand_one))
    });
    ranks.sort_by(|(hand_one, _), (hand_two, _)| {
        hand_strength(&hand_one).cmp(&hand_strength(&hand_two))
    });

    let result = &ranks
        .iter()
        .enumerate()
        .fold(0, |acc, (rank_minus_one, (_, bid))| {
            acc + ((rank_minus_one + 1) * bid)
        });
    println!("two: {}", result);
}
