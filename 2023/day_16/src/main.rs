use std::collections::HashSet;

const PUZZLIE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("puzzle one: {}", one(PUZZLIE_INPUT));
}

// if y.checked_sub(1).is_some() {
// if y + 1 < height {
// if x.checked_sub(1).is_some() {
// if *x + 1 < width {
fn move_in_direction(
    x: &usize,
    y: &usize,
    direction: &char,
    height: &usize,
    width: &usize,
) -> Option<(usize, usize)> {
    match &direction {
        'U' => {
            if y.checked_sub(1).is_some() {
                Some((*x, y - 1))
            } else {
                None
            }
        }
        'D' => {
            if *y + 1 < *height {
                Some((*x, y + 1))
            } else {
                None
            }
        }
        'L' => {
            if x.checked_sub(1).is_some() {
                Some((x - 1, *y))
            } else {
                None
            }
        }
        'R' => {
            if *x + 1 < *width {
                Some((x + 1, *y))
            } else {
                None
            }
        }
        _ => panic!("Unknown direction"),
    }
}

fn one(puzzle_input: &str) -> usize {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut world: Vec<Vec<char>> = vec![Vec::with_capacity(width); height];
    for (y, line) in puzzle_input.lines().enumerate() {
        for ch in line.chars() {
            world.get_mut(y).unwrap().push(ch);
        }
    }
    let mut energised: HashSet<(usize, usize)> = HashSet::with_capacity(width * height);
    let mut light_beams: Vec<Vec<(usize, usize, char)>> = vec![vec![(0, 0, 'R')]];
    let mut iterations: usize = 0;
    while light_beams.len() != 0 && iterations < 600 {
        let mut additions: Vec<(usize, usize, char)> = Vec::new();
        let mut dead: Vec<usize> = Vec::new();
        for (idx, light_beam) in light_beams.iter_mut().enumerate() {
            let (x, y, mut direction) = light_beam.last().unwrap().clone();
            energised.insert((x, y));
            let tile = world[y][x];
            match (&direction, tile) {
                // U == "Up"
                ('U', '/') => {
                    direction = 'R';
                }
                ('U', '-') => {
                    additions.push((x, y, 'L'));
                    direction = 'R';
                }
                ('U', '\\') => {
                    direction = 'L';
                }
                // D == "Down"
                ('D', '/') => {
                    direction = 'L';
                }
                ('D', '-') => {
                    additions.push((x, y, 'L'));
                    direction = 'R';
                }
                ('D', '\\') => {
                    direction = 'R';
                }
                // L == "Left"
                ('L', '/') => {
                    direction = 'D';
                }
                ('L', '|') => {
                    additions.push((x, y, 'U'));
                    direction = 'D';
                }
                ('L', '\\') => {
                    direction = 'U';
                }
                // R == "Right"
                ('R', '/') => {
                    direction = 'U';
                }
                ('R', '|') => {
                    additions.push((x, y, 'D'));
                    direction = 'U';
                }
                ('R', '\\') => {
                    direction = 'D';
                }
                _ => {}
            }
            if light_beam
                .iter()
                .find(|(dx, dy, _)| (dx, dy) == (&x, &y))
                .is_none()
            {
                dead.push(idx);
            }
            if let Some((new_x, new_y)) = move_in_direction(&x, &y, &direction, &height, &width) {
                light_beam.push((new_x, new_y, direction));
            } else {
                dead.push(idx);
            }
            if light_beam
                .iter()
                .find(|(dx, dy, _)| (dx, dy) == (&x, &y))
                .is_none()
            {
                dead.push(idx);
            }
        }
        for idx in dead.iter().rev() {
            light_beams.remove(*idx);
        }
        for (x, y, direction) in additions {
            if let Some((new_x, new_y)) = move_in_direction(&x, &y, &direction, &height, &width) {
                light_beams.push(vec![(new_x, new_y, direction)]);
            }
        }
    }
    energised.len()
}
