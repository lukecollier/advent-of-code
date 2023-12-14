const PUZZLE_INPUT: &str = include_str!("./input.txt");
fn main() {
    println!("puzzle one: {}", one());
}

fn one() -> usize {
    let mut summary: usize = 0;
    let mut h_world: Vec<&str> = Vec::with_capacity(PUZZLE_INPUT.len());
    let mut v_world: Vec<String> = Vec::with_capacity(PUZZLE_INPUT.len());
    let mut problem: usize = 0;
    for line in PUZZLE_INPUT.lines() {
        if line == "" {
            problem += 1;
            let mut h_reflection = 0;
            let mut v_reflection = 0;
            let mut v_size = 0;
            let mut h_size = 0;
            for amount in (2..v_world.len()).rev() {
                for (window_num, window) in v_world.windows(amount).enumerate() {
                    if let (Some(top), Some(bot)) = (
                        window.get(0..window.len() / 2),
                        window.get((window.len() / 2)..window.len()),
                    ) {
                        if top.iter().collect::<Vec<_>>() == bot.iter().rev().collect::<Vec<_>>() {
                            let reflection_at_x = amount / 2 + window_num;
                            v_reflection = reflection_at_x;
                            v_size = window.len();
                        }
                    }
                }
                if v_reflection != 0 {
                    break;
                }
            }
            for amount in (2..h_world.len()).rev() {
                if h_reflection != 0 {
                    break;
                }
                for (window_num, window) in h_world.windows(amount).enumerate() {
                    if let (Some(top), Some(bot)) = (
                        window.get(0..window.len() / 2),
                        window.get((window.len() / 2)..window.len()),
                    ) {
                        if top.iter().collect::<Vec<_>>() == bot.iter().rev().collect::<Vec<_>>() {
                            let reflection_at_x = amount / 2 + window_num;
                            h_reflection = reflection_at_x;
                            h_size = window.len();
                        }
                    }
                }
            }
            if h_size > v_size {
                summary += h_reflection * 100;
            } else if v_size >= h_size {
                summary += v_reflection;
                // v_world.iter().for_each(|print_me| println!("{}", print_me));
                // println!();
            }
            h_world.clear();
            v_world.clear();
        } else {
            h_world.push(line);
            for (idx, char) in line.char_indices() {
                if let Some(buffer) = v_world.get_mut(idx) {
                    buffer.push(char);
                } else {
                    v_world.insert(idx, char.to_string());
                }
            }
        }
    }
    summary
}
