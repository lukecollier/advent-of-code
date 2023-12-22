use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("problem one: {}", one(PUZZLE_INPUT));
}

fn one(puzzle_input: &str) -> usize {
    for (line, diagnostics_str) in puzzle_input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
    {
        let characters = vec!["#", "."];
        let diagnostics = diagnostics_str
            .split(",")
            .map(|diagnostic| diagnostic.parse::<usize>().unwrap())
            .collect_vec();
        let groups = line
            .split(".")
            .filter(|str| !str.is_empty())
            .map(|group| {
                // taken from https://stackoverflow.com/a/67746758
                let n = group.len(); // The number of combinations

                let combinations: Vec<String> = (2..n).fold(
                    characters
                        .iter()
                        .map(|c| characters.iter().map(move |&d| d.to_owned() + *c))
                        .flatten()
                        .collect(),
                    |acc, _| {
                        acc.into_iter()
                            .map(|c| characters.iter().map(move |&d| d.to_owned() + &*c))
                            .flatten()
                            .collect()
                    },
                );
                // re-inster the known broken gears
                let hash_found = group.chars().positions(|el| el == '#').collect_vec();
                if !hash_found.is_empty() {
                    let mut restored = HashSet::new();
                    for mut combination in combinations {
                        for pos in &hash_found {
                            combination.replace_range(pos..=pos, "#");
                        }
                        restored.insert(combination);
                    }
                    restored
                } else {
                    combinations.into_iter().collect::<HashSet<_>>()
                }
            })
            .collect_vec();
        let mut paths: Vec<Vec<Vec<usize>>> = Vec::with_capacity(diagnostics.len());
        for group in groups {
            println!("group");
            let mut possibilities: Vec<Vec<usize>> = Vec::with_capacity(line.len());
            for line in group {
                let mut outcome: Vec<usize> = Vec::with_capacity(line.len());
                let mut last_character = ' ';
                for ch in line.chars() {
                    let last = outcome.last_mut();
                    if ch == last_character && last.is_some() && ch == '#' {
                        *outcome.last_mut().unwrap() += 1;
                    } else if ch == '#' {
                        outcome.push(1);
                    }
                    last_character = ch;
                }
                if !outcome.is_empty() {
                    possibilities.push(outcome);
                }
                // for out in &outcome {}
            }
            dbg!(&possibilities);
            paths.push(possibilities);
        }
        let t = (0..paths.len())
            .map(|id| paths[id..paths.len()].iter().collect_vec())
            .collect_vec();
    }

    0
}

fn two(puzzle_input: &str) -> usize {
    let mut combinations = 0;
    for (springs_str, diagnostics_str) in puzzle_input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
    {
        let diagnostics = diagnostics_str
            .split(',')
            .map(|diagnostic| diagnostic.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let clusters = springs_str
            .split('.')
            .filter(|split| !split.is_empty())
            .collect::<Vec<_>>();
        for start in 0..clusters.len() {
            let mut diagnostics_iter = diagnostics.iter();
            let mut diagnostic = diagnostics_iter.next();
            let mut additional_combinations = 0;
            for cluster in clusters.iter().skip(start) {
                let current = diagnostic.unwrap();
                let mut wildcards = cluster.chars().filter(|char| char == &'?').count();
                let broken = cluster.len() - wildcards;
                if current == &(broken + wildcards) {
                    diagnostic = diagnostics_iter.next();
                } else if current < &wildcards {
                    additional_combinations += wildcards / current;
                    dbg!(additional_combinations, wildcards, current);
                    while wildcards > 0 {
                        wildcards = wildcards.checked_sub(current + 1).unwrap_or(0);
                        diagnostic = diagnostics_iter.next();
                    }
                } else if current == &wildcards && broken == 0 {
                    diagnostic = diagnostics_iter.next();
                } else if broken + wildcards > *current {
                    let mut groups: Vec<String> = Vec::with_capacity(cluster.len());
                    let mut buffer = String::with_capacity(cluster.len());
                    let mut iter = cluster.chars();
                    let mut cur = iter.next().unwrap();
                    for ch in cluster.chars() {
                        if ch != cur {
                            groups.push(buffer.clone());
                            buffer.clear();
                        }
                        cur = ch;
                        buffer.push(ch);
                    }
                    groups.push(buffer.clone());
                    let first = groups.first().unwrap();
                    let last = groups.last().unwrap();
                    // now we need to find the first and last, if they're #
                }
            }
            if diagnostic.is_none() {
                combinations += 1;
                combinations += additional_combinations;
                break;
            }
        }
    }
    combinations
}
