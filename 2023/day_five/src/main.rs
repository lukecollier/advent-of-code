const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    one();
    two();
}

fn one() {
    let mut lines = PUZZLE_INPUT.lines();
    let mut seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut reading_map = false;
    let mut conversion: Vec<(usize, usize, usize)> = Vec::new();
    for line in lines {
        if line.contains(':') {
            reading_map = true;
        } else if line.trim().is_empty() {
            for idx in 0..seeds.len() {
                let seed = seeds[idx];

                if let Some((source_min, _, destination)) = conversion
                    .iter()
                    .find(|(source_min, source_max, _)| source_min <= &seed && seed <= *source_max)
                {
                    let add_this = source_min.abs_diff(seed);
                    seeds[idx] = destination + add_this;
                }
            }
            reading_map = false;
            conversion.clear();
        } else if reading_map {
            let mut split = line.split_ascii_whitespace();
            let destination = split.next().unwrap().parse::<usize>().unwrap();
            let source = split.next().unwrap().parse::<usize>().unwrap();
            let range = split.next().unwrap().parse::<usize>().unwrap();
            conversion.push((source, source + range, destination));
        }
    }
    seeds.sort();
    println!("problem one: {}", seeds.first().unwrap());
}

fn two() {
    let mut lines = PUZZLE_INPUT.lines();
    let found_seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut seeds = Vec::new();
    // todo: The optimisation should happen here, it's a data structure fix. I.e we store the range
    // not the raw values, we can then use the range overlaps with the range conversion tables to
    // quickly generate the right number of seeds
    for seed_pair in found_seeds.chunks(2) {
        let start = seed_pair.first().unwrap().to_owned();
        let range = seed_pair.last().unwrap().to_owned();
        for seed in start..(start + range) {
            seeds.push(seed);
        }
    }
    let mut reading_map = false;
    let mut conversion: Vec<(usize, usize, usize)> = Vec::new();
    for line in lines {
        if line.contains(':') {
            reading_map = true;
        } else if line.trim().is_empty() {
            for seed in seeds.iter_mut() {
                if let Some((source_min, _, destination)) = conversion
                    .iter()
                    .find(|(source_min, source_max, _)| source_min <= seed && source_max >= seed)
                {
                    let add_this = source_min.abs_diff(*seed);
                    *seed = destination + add_this;
                }
            }
            reading_map = false;
            conversion.clear();
        } else if reading_map {
            let mut split = line.split_ascii_whitespace();
            let destination = split.next().unwrap().parse::<usize>().unwrap();
            let source = split.next().unwrap().parse::<usize>().unwrap();
            let range = split.next().unwrap().parse::<usize>().unwrap();
            conversion.push((source, source + range, destination));
        }
    }
    seeds.sort();
    println!("problem two: {}", seeds.first().unwrap());
}
