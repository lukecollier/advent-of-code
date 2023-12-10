use core::f32;
use std::collections::HashSet;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("puzzle one: {}", one());
}

fn has_up(pipe: &char) -> bool {
    match pipe {
        '|' => true,
        'L' => true,
        'J' => true,
        'S' => true,
        _ => false,
    }
}
fn has_down(pipe: &char) -> bool {
    match pipe {
        '|' => true,
        'F' => true,
        '7' => true,
        'S' => true,
        _ => false,
    }
}
fn has_left(pipe: &char) -> bool {
    match pipe {
        '-' => true,
        'J' => true,
        '7' => true,
        'S' => true,
        _ => false,
    }
}
fn has_right(pipe: &char) -> bool {
    match pipe {
        '-' => true,
        'L' => true,
        'F' => true,
        'S' => true,
        _ => false,
    }
}
// | is a vertical pipe connecting north and south. ║
// - is a horizontal pipe connecting east and west. ═
// L is a 90-degree bend connecting north and east. ╚
// J is a 90-degree bend connecting north and west. ╝
// 7 is a 90-degree bend connecting south and west. ╗
// F is a 90-degree bend connecting south and east. ╔
fn is_connected_above(pipe: &char, other: &char) -> bool {
    has_up(pipe) && has_down(other)
}

fn is_connected_below(pipe: &char, other: &char) -> bool {
    has_down(pipe) && has_up(other)
}

fn is_connected_left(pipe: &char, other: &char) -> bool {
    has_left(pipe) && has_right(other)
}

fn is_connected_right(pipe: &char, other: &char) -> bool {
    has_right(pipe) && has_left(other)
}

fn one() -> usize {
    let mut y_x_world: Vec<Vec<char>> = Vec::new();
    let mut start_at: Option<(usize, usize)> = None;
    for (y, line) in PUZZLE_INPUT.lines().enumerate() {
        let chars = Vec::with_capacity(line.len());
        y_x_world.push(chars);
        for (x, char) in line.char_indices() {
            y_x_world.get_mut(y).unwrap().push(char);
            if char == 'S' {
                start_at = Some((x, y));
            }
        }
    }
    if let Some((x, y)) = start_at {
        let mut locations: Vec<(usize, usize, usize)> = Vec::new();
        if let Some(up) = y_x_world.get(y - 1).and_then(|col| col.get(x)) {
            if is_connected_above(&'S', up) {
                locations.push((x, y - 1, 1));
            }
        }
        if let Some(right) = y_x_world.get(y).and_then(|col| col.get(x + 1)) {
            if is_connected_right(&'S', right) {
                locations.push((x + 1, y, 1));
            }
        }
        if let Some(down) = y_x_world.get(y + 1).and_then(|col| col.get(x)) {
            if is_connected_below(&'S', down) {
                locations.push((x, y + 1, 1));
            }
        }
        if let Some(left) = y_x_world.get(y).and_then(|col| col.get(x - 1)) {
            if is_connected_left(&'S', left) {
                locations.push((x - 1, y, 1));
            }
        }
        let mut visited: HashSet<(usize, usize, &char)> = HashSet::with_capacity(10);
        visited.insert((x, y, &'S'));
        loop {
            for idx in 0..locations.len() {
                let (x, y, steps) = locations[idx];
                let current = y_x_world.get(y).unwrap().get(x).unwrap();
                visited.insert((x, y, current));
                let mut moved = false;
                if let Some(up) = y
                    .checked_sub(1)
                    .and_then(|y| y_x_world.get(y).and_then(|col| col.get(x)))
                {
                    if is_connected_above(current, up)
                        && !visited.contains(&(x, y - 1, up))
                        && !moved
                    {
                        locations[idx] = (x, y - 1, steps + 1);
                        moved = true;
                    }
                }
                if let Some(right) = y_x_world.get(y).and_then(|col| col.get(x + 1)) {
                    if is_connected_right(current, right)
                        && !visited.contains(&(x + 1, y, right))
                        && !moved
                    {
                        locations[idx] = (x + 1, y, steps + 1);
                        moved = true;
                    }
                }
                if let Some(down) = y_x_world.get(y + 1).and_then(|col| col.get(x)) {
                    if is_connected_below(current, down)
                        && !visited.contains(&(x, y + 1, down))
                        && !moved
                    {
                        locations[idx] = (x, y + 1, steps + 1);
                        moved = true;
                    }
                }
                if let Some(left) = y_x_world
                    .get(y)
                    .and_then(|col| x.checked_sub(1).and_then(|x| col.get(x)))
                {
                    if is_connected_left(current, left)
                        && !visited.contains(&(x - 1, y, left))
                        && !moved
                    {
                        locations[idx] = (x - 1, y, steps + 1);
                    }
                }
            }
            if locations.windows(2).fold(true, |acc, values| {
                acc && values.first().unwrap() == values.last().unwrap()
            }) {
                return locations.get(0).unwrap().2;
            }
        }
    }
    0
}
