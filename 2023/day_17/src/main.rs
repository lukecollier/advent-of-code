use std::path;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("puzzle input {}", one(PUZZLE_INPUT));
}

fn one(puzzle_input: &str) -> usize {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().len();
    let mut world: Vec<Vec<u32>> = vec![vec![0; width]; height];
    for (y, line) in puzzle_input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            *world.get_mut(x).unwrap().get_mut((height - 1) - y).unwrap() =
                ch.to_digit(10).unwrap();
        }
    }
    let mut search_paths: Vec<(Vec<(usize, usize)>, usize, usize)> =
        vec![(vec![(0, height - 1)], 0, 3)];
    let mut completed_paths: Vec<(Vec<(usize, usize)>, usize, usize)> = Vec::new();
    while search_paths.len() > 0 {
        let mut additional_search_paths: Vec<(Vec<(usize, usize)>, usize, usize)> = Vec::new();
        let mut completed_search_paths: Vec<usize> = Vec::new();
        // todo: X moves is actually consecutive moves remaining
        for (idx, (paths, weight, consecutive_moves)) in search_paths.iter_mut().enumerate() {
            let (x, y) = paths.last().unwrap();
            let (previous_x, previous_y) =
                paths.get(paths.len().checked_sub(0).unwrap_or(0)).unwrap();
            if y != &0 && x != &12 {
                let mut new_positions: Vec<((usize, usize), u32, usize)> = Vec::with_capacity(8);
                if previous_y.abs_diff(*y) == 1 && consecutive_moves != &0 {
                    let _top_centre = world.get(*x).and_then(|row| {
                        row.get(*y + 1)
                            .map(|cost| new_positions.push(((*x, *y + 1), *cost, 3)))
                    });
                    let _bot_centre = y.checked_sub(1).and_then(|checked_y| {
                        world
                            .get(*x)
                            .and_then(|row| row.get(checked_y))
                            .map(|cost| new_positions.push(((*x, checked_y), *cost, 3)))
                    });
                }
                if previous_x.abs_diff(*x) == 1 && consecutive_moves != &0 {
                    let _centre_left = x.checked_sub(1).and_then(|checked_x| {
                        world.get(checked_x).and_then(|row| {
                            row.get(*y)
                                .map(|cost| new_positions.push(((checked_x, *y), *cost, 3)))
                        })
                    });
                    let _centre_right = world.get(*x + 1).and_then(|row| {
                        row.get(*y)
                            .map(|cost| new_positions.push(((*x + 1, *y), *cost, 3)))
                    });
                }

                let mut new_positions_iter = new_positions.iter();
                let (pos, additional_weight, x_moves) = new_positions_iter.next().unwrap();
                let mut new_search_paths = new_positions_iter
                    .filter_map(|(pos, additional_weight, x_moves)| {
                        let mut new_path = paths.clone();
                        if !new_path.contains(pos) {
                            new_path.push(*pos);
                            Some((new_path, *weight + *additional_weight as usize, *x_moves))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if !paths.contains(pos) {
                    paths.push(*pos);
                    *weight += *additional_weight as usize;
                    *consecutive_moves = *x_moves;
                } else {
                    completed_search_paths.push(idx);
                }
                if new_search_paths.len() > 0 {
                    additional_search_paths.append(&mut new_search_paths);
                } else {
                    if !completed_search_paths.contains(&idx) {
                        completed_search_paths.push(idx);
                    }
                }
            } else {
                if !completed_search_paths.contains(&idx) {
                    completed_search_paths.push(idx);
                }
            }
        }
        println!("{}", search_paths.len());
        for new_search_path in additional_search_paths {
            search_paths.push(new_search_path);
        }
        for completed_id in completed_search_paths.iter().rev() {
            let completed = search_paths.remove(*completed_id);
            completed_paths.push(completed);
        }
    }
    for m in completed_paths {
        let last = m.0.last().unwrap();
        if last.1 == 0 && last.0 >= 3 {
            println!("{:?}", last);
        }
    }
    // completed_paths.iter().filter_map(|m| m.0.last().unwrap().0 == );
    0
}

fn two(puzzle_input: &str) -> usize {
    0
}
