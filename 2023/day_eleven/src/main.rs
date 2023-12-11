use std::collections::{HashMap, HashSet};

const PUZZLE_INPUT: &str = include_str!("./input.txt");
const ONE_MILLION: usize = 1;

fn main() {
    println!("problem one: {}", one());
    println!("problem two: {}", two());
}

fn one() -> usize {
    let mut universe: Vec<Vec<usize>> = Vec::new();

    let mut id: usize = 0;
    for line in PUZZLE_INPUT.lines() {
        let row = line
            .chars()
            .map(|char| {
                if char == '#' {
                    id += 1;
                    id
                } else {
                    0
                }
            })
            .collect::<Vec<_>>();
        //expand in the x
        if !line.contains('#') {
            universe.push(row.clone());
        }
        universe.push(row);
    }
    //expand in the y
    let width = universe.first().unwrap().len();
    let mut columns_to_duplicate: Vec<usize> = Vec::with_capacity(width);
    for x in 0..width {
        let mut universe_found = false;
        for y in 0..universe.len() {
            let column = universe.get(y).unwrap();
            let value = column.get(x).unwrap();
            if *value > 0 {
                universe_found = true;
                break;
            }
        }
        if !universe_found {
            columns_to_duplicate.push(x);
        }
    }
    for (offset, x) in columns_to_duplicate.iter().enumerate() {
        for y in 0..universe.len() {
            universe[y].insert(x + offset, 0);
        }
    }
    let mut galaxies_positions: Vec<(usize, usize)> = Vec::with_capacity(id);
    for x in 0..universe.first().unwrap().len() {
        for y in 0..universe.len() {
            if universe[y][x] > 0 {
                galaxies_positions.push((x, y));
            }
        }
    }
    // A map of the node node_id -> node_id -> distancce
    let mut edges: HashMap<(usize, usize), usize> = HashMap::with_capacity(id * id);
    for (x_one, y_one) in &galaxies_positions {
        for (x_two, y_two) in &galaxies_positions {
            if (x_one, y_one) != (x_two, y_two) {
                let distance = x_one.abs_diff(*x_two) + y_one.abs_diff(*y_two);
                let from = universe[*y_one][*x_one];
                let too = universe[*y_two][*x_two];
                edges
                    .entry((from.min(too), from.max(too)))
                    .or_insert(distance);
            }
        }
    }
    edges.iter().fold(0, |acc, (_, distance)| acc + distance)
}

fn two() -> usize {
    let mut galaxies: Vec<((usize, usize), usize)> = Vec::with_capacity(1000);
    let mut universe: Vec<Vec<usize>> = Vec::with_capacity(PUZZLE_INPUT.len());
    let mut galaxy_id = 0;
    let mut y_offset = 0;
    for (y, line) in PUZZLE_INPUT.lines().enumerate() {
        if line.chars().all(|char| char == '.') {
            y_offset += ONE_MILLION;
        }
        let mut row: Vec<usize> = Vec::with_capacity(line.len());
        for (x, char) in line.char_indices() {
            if char == '#' {
                galaxy_id += 1;
                row.push(galaxy_id);
                galaxies.push(((x, y + y_offset), galaxy_id));
            } else {
                row.push(0);
            }
        }
        universe.push(row);
    }
    let width = universe.first().unwrap().len();
    let mut columns_to_duplicate: Vec<usize> = Vec::with_capacity(width);
    for x in 0..width {
        let mut universe_found = false;
        for y in 0..universe.len() {
            let column = universe.get(y).unwrap();
            let value = column.get(x).unwrap();
            if *value > 0 {
                universe_found = true;
                break;
            }
        }
        if !universe_found {
            columns_to_duplicate.push(x);
        }
    }
    for idx in 0..galaxies.len() {
        let x = galaxies[idx].0 .0;
        let y = galaxies[idx].0 .1;
        let x_offset = columns_to_duplicate
            .clone()
            .into_iter()
            .filter(|threshold| threshold < &x)
            .count()
            * ONE_MILLION;
        galaxies[idx].0 .0 += x_offset;
    }
    // A map of the node node_id -> node_id -> distancce
    let mut edges: HashSet<(usize, usize)> = HashSet::with_capacity(galaxy_id * galaxy_id);
    let mut distance_sum = 0;
    for ((x_from, y_from), from) in &galaxies {
        for ((x_to, y_to), to) in &galaxies {
            dbg!(x_from, x_to);
            let key = (*from.min(to), *from.max(to));
            if (x_from, y_from) != (x_to, y_to) && !edges.contains(&key) {
                let distance = x_from.abs_diff(*x_to) + y_from.abs_diff(*y_to);
                distance_sum += distance;
                edges.insert(key);
            }
        }
    }
    distance_sum
}
