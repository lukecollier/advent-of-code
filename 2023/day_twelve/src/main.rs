use std::collections::HashSet;

use itertools::Itertools;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("problem one: {}", one(PUZZLE_INPUT));
}

fn one(puzzle_input: &str) -> usize {
    let mut found = 0;
    for (line, diagnostics_str) in puzzle_input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
    {
        let diagnostics = diagnostics_str
            .split(",")
            .map(|diagnostic| diagnostic.parse::<usize>().unwrap())
            .collect_vec();

        let diagnostics_sum = diagnostics.iter().sum::<usize>();

        let combination = line
            .chars()
            .map(|ch| match ch {
                '.' => vec!['.'],
                '?' => vec!['#', '.'],
                '#' => vec!['#'],
                _ => todo!(),
            })
            .multi_cartesian_product()
            .map(|str| str.iter().collect::<String>())
            .filter(|line| {
                line.chars()
                    .fold(0, |acc, ch| if ch == '#' { acc + 1 } else { acc })
                    >= diagnostics_sum
            })
            .collect_vec();
        for subject in combination {
            let mut counts: Vec<usize> = Vec::with_capacity(subject.len());
            let mut last_ch = ' ';
            for ch in subject.chars() {
                if ch == '#' && last_ch == '#' {
                    *counts.last_mut().unwrap() += 1;
                } else if ch == '#' {
                    counts.push(1);
                }
                last_ch = ch;
            }
            if &diagnostics == &counts {
                found += 1;
            }
        }
    }

    found
}
