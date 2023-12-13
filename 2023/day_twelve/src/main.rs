use std::collections::HashSet;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("problem one: {}", two(PUZZLE_INPUT));
}

fn one(puzzle_input: &str) -> usize {
    let mut combinations = 0;
    for (springs_str, diagnostics_str) in puzzle_input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
    {
        let diagnostics = diagnostics_str
            .split(',')
            .map(|diagnostic| diagnostic.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let end_start_bonus = 2;
        let min_chars_needed: usize =
            diagnostics.iter().sum::<usize>() + (diagnostics.iter().count() - end_start_bonus);
        let mut start = 0;
        while start <= springs_str.len() - min_chars_needed {
            let mut diagnostics_iter = diagnostics.iter();
            let mut diagnostic_opt = diagnostics_iter.next();
            let mut pos = start;
            while let Some(diagnostic) = diagnostic_opt {
                if let Some(current) = &springs_str.get(pos..pos + diagnostic) {
                    let end = springs_str
                        .get(pos + diagnostic..pos + diagnostic + 1)
                        .unwrap_or(".");
                    if !current.contains('.') && end == "." {
                        diagnostic_opt = diagnostics_iter.next();
                        pos += diagnostic;
                    } else if !current.contains('.') && end != "." {
                        // we need to skip + count the unique combinations when we reach a block of ?
                        diagnostic_opt = diagnostics_iter.next();
                        pos += diagnostic + 1;
                    } else {
                        pos += 1;
                    }
                } else {
                    break;
                }
            }
            if diagnostic_opt.is_none() {
                combinations += 1;
            }
            start += 1;
        }
    }
    combinations
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
                    additional_combinations += wildcards / (current + 1);
                    dbg!(wildcards, additional_combinations, cluster);
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
