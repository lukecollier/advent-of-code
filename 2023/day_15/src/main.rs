use indexmap::IndexMap;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("puzzle one: {}", one(PUZZLE_INPUT.trim()));
    println!("puzzle two: {}", two(PUZZLE_INPUT.trim()));
}

fn hash(str: &str) -> usize {
    let mut current = 0;
    for ch in str.chars() {
        current += ch as usize;
        current *= 17;
        current %= 256;
    }
    current
}

fn one(puzzle_input: &str) -> usize {
    let mut sum = 0;
    for str in puzzle_input.split(",") {
        sum += hash(str.trim());
    }
    sum
}

fn two(puzzle_input: &str) -> usize {
    let mut sum = 0;
    let mut total_boxes = 0;
    for str in puzzle_input.split(",") {
        let mut label = String::with_capacity(str.len());
        let mut is_label = true;
        let mut number_str = String::new();
        for ch in str.chars() {
            if ch.is_ascii_punctuation() {
                is_label = false;
            } else if is_label {
                label.push(ch);
            }
        }
        total_boxes = total_boxes.max(hash(&label));
    }

    let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); total_boxes + 1];
    for str in puzzle_input.split(",") {
        let (label, number_str) = str
            .split_once(|ch: char| ch.is_ascii_punctuation())
            .unwrap();
        let box_number = hash(&label.trim());
        // we set
        if !number_str.is_empty() {
            let number = number_str.parse::<u8>().unwrap();
            let mut replaced = false;
            for (key, value) in boxes.get_mut(box_number).unwrap() {
                if key == &label {
                    *value = number;
                    replaced = true;
                }
            }
            if !replaced {
                boxes.get_mut(box_number).unwrap().push((&label, number));
            }
        } else {
            let current = boxes.get(box_number).unwrap();
            let mut idx = 0;
            for (key, _) in current.iter() {
                if key == &label {
                    boxes.get_mut(box_number).unwrap().remove(idx);
                    break;
                }
                idx += 1;
            }
        }
    }
    for (box_pos, map) in boxes.iter().enumerate() {
        for (slot_number, (_key, focal)) in map.iter().enumerate() {
            sum += (slot_number + 1) * *focal as usize * (box_pos + 1)
        }
    }
    sum
}
