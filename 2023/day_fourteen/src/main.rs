const PUZZLE_INPUT: &str = include_str!("./test.txt");

fn main() {
    println!("puzzle one {}", one());
    println!("puzzle two {}", two());
}

fn one() -> usize {
    let mut lines = PUZZLE_INPUT.lines();
    let first_line = lines.next().unwrap();
    let mut weight = 0;
    let mut rolling: Vec<usize> = vec![0; first_line.len()];
    let mut total = 0;
    for (idx, line) in PUZZLE_INPUT.lines().rev().enumerate() {
        for (pos, ch) in line.char_indices() {
            match ch {
                '#' => {
                    weight += (idx - *rolling.get(pos).unwrap() + 1..idx + 1).sum::<usize>();
                    *rolling.get_mut(pos).unwrap() = 0;
                }
                'O' => {
                    *rolling.get_mut(pos).unwrap() += 1;
                }
                '.' => {}
                _ => todo!(),
            }
        }
        total += 1;
    }
    for rolled in rolling {
        weight += (total - rolled + 1..total + 1).sum::<usize>();
    }
    weight
}

fn two() -> usize {
    let mut lines = PUZZLE_INPUT.lines();
    let first_line = lines.next().unwrap();
    let mut world: Vec<Vec<char>> = vec![vec![' '; first_line.len()]; first_line.len()];
    let mut total = 0;
    for (y, line) in PUZZLE_INPUT.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            *world.get_mut(x).unwrap().get_mut(y).unwrap() = ch;
        }
        total += 1;
    }
    world.iter().for_each(|row| {
        row.iter().for_each(|ch| {
            print!("{}", ch);
        });
        println!();
    });
    0
}
