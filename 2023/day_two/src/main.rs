fn main() {
    puzzle_one();
    puzzle_two();
}

fn puzzle_one() {
    let puzzle_input = include_str!("./input.txt").trim().lines();
    let mut possible_games = 0;
    for line in puzzle_input {
        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;
        for (pos, char) in line.char_indices().skip_while(|(_, char)| char != &':') {
            if char == 'r' && &line[pos..(pos + 3)] == "red" {
                let num_red = line[(pos - 3)..(pos - 1)]
                    .trim_start()
                    .parse::<u32>()
                    .unwrap();
                red_max = red_max.max(num_red);
            } else if char == 'b' && &line[pos..(pos + 4)] == "blue" {
                let num_blue = line[(pos - 3)..(pos - 1)]
                    .trim_start()
                    .parse::<u32>()
                    .unwrap();
                blue_max = blue_max.max(num_blue);
            } else if char == 'g' && &line[pos..(pos + 5)] == "green" {
                let num_green = line[(pos - 3)..(pos - 1)]
                    .trim_start()
                    .parse::<u32>()
                    .unwrap();
                green_max = green_max.max(num_green);
            }
        }
        if blue_max <= 14 && green_max <= 13 && red_max <= 12 {
            let game_number = line
                .chars()
                .into_iter()
                .skip("Game ".len())
                .take_while(|char| char != &':')
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            possible_games += game_number;
        }
    }
    println!("puzzle 1: {}", possible_games)
}

fn puzzle_two() {
    let puzzle_input = include_str!("./input.txt").trim().lines();
    let mut game_power = 0;
    for line in puzzle_input {
        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;
        for (pos, char) in line.char_indices().skip_while(|(_, char)| char != &':') {
            if char == 'r' && &line[pos..(pos + 3)] == "red" {
                let num_red = line[(pos - 3)..(pos - 1)]
                    .trim_start()
                    .parse::<u32>()
                    .unwrap();
                red_max = red_max.max(num_red);
            } else if char == 'b' && &line[pos..(pos + 4)] == "blue" {
                let num_blue = line[(pos - 3)..(pos - 1)]
                    .trim_start()
                    .parse::<u32>()
                    .unwrap();
                blue_max = blue_max.max(num_blue);
            } else if char == 'g' && &line[pos..(pos + 5)] == "green" {
                let num_green = line[(pos - 3)..(pos - 1)]
                    .trim_start()
                    .parse::<u32>()
                    .unwrap();
                green_max = green_max.max(num_green);
            }
        }
        game_power += blue_max * green_max * red_max;
    }
    println!("puzzle 2: {}", game_power)
}
