const PUZZLE_INPUT: &str = include_str!("./input.txt");

fn main() {
    for i in 0..-5 {
        dbg!(i);
    }
    println!("puzzle input {}", one(PUZZLE_INPUT));
}

struct Line {
    from: (usize, usize, usize),
    to: (usize, usize, usize),
}

impl Line {
    fn new(from: (usize, usize, usize), to: (usize, usize, usize)) -> Line {
        Line { from, to }
    }

    fn intersect(&self, other: &Line) -> bool {
        let (other_tx, other_ty, other_tz) = other.to;
        let (other_fx, other_fy, other_fz) = other.from;
        let (tx, ty, tz) = other.to;
        let (fx, fy, fz) = other.from;
        other_tx..other_fx;
        other_ty..other_fy;
        other_tz..other_fz;
        tx..fx;
        ty..fy;
        tz..fz;
        false
    }
}

fn one(puzzle_input: &str) -> usize {
    let mut world: Vec<Line> = Vec::with_capacity(PUZZLE_INPUT.lines().count());
    for line in puzzle_input.lines() {
        let (from_line, to_line) = line.split_once("~").unwrap();
        let mut from_iter = from_line
            .split(",")
            .map(|str| str.parse::<usize>().unwrap());
        let mut to_iter = to_line.split(",").map(|str| str.parse::<usize>().unwrap());
        let from = (
            from_iter.next().unwrap(),
            from_iter.next().unwrap(),
            from_iter.next().unwrap(),
        );
        let to = (
            to_iter.next().unwrap(),
            to_iter.next().unwrap(),
            to_iter.next().unwrap(),
        );
        world.push(Line::new(from, to));
    }
    for line in puzzle_input.lines() {}
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_find_intersect() {
        let line1 = Line::new((0, 1, 0), (4, 1, 0));
        let line2 = Line::new((0, 5, 0), (0, 0, 0));
        assert_eq!(line1.intersect(&line2), true);
    }
}
