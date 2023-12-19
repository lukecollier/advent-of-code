use std::collections::HashSet;

const PUZZLIE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("one puzzle: {}", one(PUZZLIE_INPUT));
    println!("two puzzle: {}", two(PUZZLIE_INPUT));
}

fn get(world: &Vec<Vec<usize>>, x: usize, y: usize) -> Option<&usize> {
    world.get(x).and_then(|col| col.get(y))
}

// plan, phase 1 is to draw the line then simply do a scan of every row of the world
fn one(puzzle_input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut max_height = 0;
    let mut min_height = 0;
    let mut max_width = 0;
    let mut min_width = 0;
    for line in puzzle_input.lines() {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().unwrap().parse::<char>().unwrap();
        let magnitude = split.next().unwrap().parse::<isize>().unwrap();
        if direction == 'U' {
            y -= magnitude;
            max_height = y.max(max_height);
            min_height = y.min(min_height);
        } else if direction == 'D' {
            y += magnitude;
            max_height = y.max(max_height);
            min_height = y.min(min_height);
        }
        if direction == 'L' {
            x -= magnitude;
            min_width = x.min(min_width);
            max_width = x.max(max_width);
        } else if direction == 'R' {
            x += magnitude;
            min_width = x.min(min_width);
            max_width = x.max(max_width);
        }
    }
    let height = min_height.abs_diff(max_height) + 1;
    let width = min_width.abs_diff(max_width) + 1;
    let (mut x, mut y): (usize, usize) = (min_width.abs() as usize, min_height.abs() as usize);
    let mut world: Vec<Vec<usize>> = vec![vec![0; height]; width];
    let mut moved = 0;
    for line in puzzle_input.lines() {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().unwrap().parse::<char>().unwrap();
        let magnitude = split.next().unwrap().parse::<isize>().unwrap();
        for _ in 0..magnitude {
            match direction {
                'R' => x += 1,
                'L' => x -= 1,
                'U' => y -= 1,
                'D' => y += 1,
                _ => todo!(),
            }
            moved += 1;
            *world.get_mut(x).unwrap().get_mut(y).unwrap() = 1;
        }
    }
    for (wx, column) in world.iter().enumerate() {
        for (wy, value) in column.iter().enumerate() {
            if value == &0 {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut search: Vec<(usize, usize)> = vec![(wx, wy)];
                let mut next: Vec<(usize, usize)> = Vec::new();
                let mut semaphore = true;
                while (&search).len() != 0 && semaphore {
                    for (x, y) in &search {
                        let up_opt = get(&world, *x, y + 1);
                        let down_opt = y
                            .checked_sub(1)
                            .and_then(|checked_y| get(&world, *x, checked_y));
                        let left_opt = x
                            .checked_sub(1)
                            .and_then(|checked_x| get(&world, checked_x, *y));
                        let right_opt = get(&world, x + 1, *y);
                        // if this occurs we're at an edge and not actually in our fill
                        if let Some((((up, down), left), right)) =
                            up_opt.zip(down_opt).zip(left_opt).zip(right_opt)
                        {
                            if up == &0 && !visited.contains(&(*x, y + 1)) {
                                next.push((*x, y + 1));
                                visited.insert((*x, y + 1));
                            }
                            if down == &0 && !visited.contains(&(*x, y - 1)) {
                                next.push((*x, y - 1));
                                visited.insert((*x, y - 1));
                            }
                            if left == &0 && !visited.contains(&(x - 1, *y)) {
                                next.push((x - 1, *y));
                                visited.insert((x - 1, *y));
                            }
                            if right == &0 && !visited.contains(&(x + 1, *y)) {
                                next.push((x + 1, *y));
                                visited.insert((x + 1, *y));
                            }
                        } else {
                            semaphore = false;
                            break;
                        }
                    }
                    for _ in 0..search.len() {
                        visited.insert(search.pop().unwrap());
                    }
                    search.append(&mut next);
                }
                if semaphore {
                    return visited.len() + moved;
                }
            }
        }
    }
    width * height
}

fn two(puzzle_input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut max_height = 0;
    let mut min_height = 0;
    let mut max_width = 0;
    let mut min_width = 0;
    for line in puzzle_input.lines() {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().unwrap().parse::<char>().unwrap();
        let magnitude = split.next().unwrap().parse::<isize>().unwrap();
        if direction == 'U' {
            y -= magnitude;
            max_height = y.max(max_height);
            min_height = y.min(min_height);
        } else if direction == 'D' {
            y += magnitude;
            max_height = y.max(max_height);
            min_height = y.min(min_height);
        }
        if direction == 'L' {
            x -= magnitude;
            min_width = x.min(min_width);
            max_width = x.max(max_width);
        } else if direction == 'R' {
            x += magnitude;
            min_width = x.min(min_width);
            max_width = x.max(max_width);
        }
    }
    let height = min_height.abs_diff(max_height) + 1;
    let width = min_width.abs_diff(max_width) + 1;
    let (mut x, mut y): (usize, usize) = (min_width.abs() as usize, min_height.abs() as usize);
    let mut world: Vec<Vec<usize>> = vec![vec![0; height]; width];
    let mut moved = 0;
    for line in puzzle_input.lines() {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().unwrap().parse::<char>().unwrap();
        let magnitude = split.next().unwrap().parse::<isize>().unwrap();
        for _ in 0..magnitude {
            match direction {
                'R' => x += 1,
                'L' => x -= 1,
                'U' => y -= 1,
                'D' => y += 1,
                _ => todo!(),
            }
            moved += 1;
            *world.get_mut(x).unwrap().get_mut(y).unwrap() = 1;
        }
    }
    for (wx, column) in world.iter().enumerate() {
        for (wy, value) in column.iter().enumerate() {
            if value == &0 {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut search: Vec<(usize, usize)> = vec![(wx, wy)];
                let mut next: Vec<(usize, usize)> = Vec::new();
                let mut semaphore = true;
                while (&search).len() != 0 && semaphore {
                    for (x, y) in &search {
                        let up_opt = get(&world, *x, y + 1);
                        let down_opt = y
                            .checked_sub(1)
                            .and_then(|checked_y| get(&world, *x, checked_y));
                        let left_opt = x
                            .checked_sub(1)
                            .and_then(|checked_x| get(&world, checked_x, *y));
                        let right_opt = get(&world, x + 1, *y);
                        // if this occurs we're at an edge and not actually in our fill
                        if let Some((((up, down), left), right)) =
                            up_opt.zip(down_opt).zip(left_opt).zip(right_opt)
                        {
                            if up == &0 && !visited.contains(&(*x, y + 1)) {
                                next.push((*x, y + 1));
                                visited.insert((*x, y + 1));
                            }
                            if down == &0 && !visited.contains(&(*x, y - 1)) {
                                next.push((*x, y - 1));
                                visited.insert((*x, y - 1));
                            }
                            if left == &0 && !visited.contains(&(x - 1, *y)) {
                                next.push((x - 1, *y));
                                visited.insert((x - 1, *y));
                            }
                            if right == &0 && !visited.contains(&(x + 1, *y)) {
                                next.push((x + 1, *y));
                                visited.insert((x + 1, *y));
                            }
                        } else {
                            semaphore = false;
                            break;
                        }
                    }
                    for _ in 0..search.len() {
                        visited.insert(search.pop().unwrap());
                    }
                    search.append(&mut next);
                }
                if semaphore {
                    return visited.len() + moved;
                }
            }
        }
    }
    width * height
}
