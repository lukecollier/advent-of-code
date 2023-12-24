use std::collections::HashMap;

const INPUT_PUZZLE: &str = include_str!("./input.txt");

fn main() {
    println!("problem one: {}", one(INPUT_PUZZLE));
}
fn signal_to_str(signal: bool) -> String {
    if signal {
        "high".to_string()
    } else {
        "low".to_string()
    }
}

fn one(puzzle_input: &str) -> usize {
    let mut states: HashMap<&str, bool> = HashMap::with_capacity(puzzle_input.len());
    let mut world: HashMap<&str, Vec<&str>> = HashMap::with_capacity(puzzle_input.len());
    let mut conjunction_signals: HashMap<&str, Vec<&str>> =
        HashMap::with_capacity(puzzle_input.len());
    for line in puzzle_input.lines().rev() {
        let (module, output) = line.split_once(" -> ").unwrap();
        if module.starts_with("%") {
            let label = &module[1..];
            let destinations = output.split(", ").collect::<Vec<_>>();
            states.entry(label).or_insert(false);
            for dest in &destinations {
                conjunction_signals
                    .entry(dest)
                    .and_modify(|pulses| pulses.push(label));
            }
            world.insert(label, destinations);
        } else if module.starts_with("&") {
            // conjunction
            let label = &module[1..];
            let destinations = output.split(",").collect::<Vec<_>>();
            conjunction_signals.entry(label).or_insert(Vec::new());
            world.insert(label, destinations);
        } else if module == "broadcaster" {
            let destinations = output.split(", ").collect::<Vec<_>>();
            world.insert("broadcaster", destinations);
        }
    }
    // start at one from button -> broadcaster
    let mut low_signals = 0;
    let mut high_signals = 0;
    for _ in 1..=1000 {
        let mut targets = vec![("broadcaster", false)];
        low_signals += 1;
        // println!("button -low-> broadcaster");
        while !targets.is_empty() {
            let mut next: Vec<(&str, bool)> = Vec::with_capacity(1);
            for (key, current_signal) in &targets {
                if let Some(destinations) = world.get_mut(key) {
                    if let Some(signals) = conjunction_signals.get_mut(key) {
                        if signals.len() == 1 {
                            for dest in destinations {
                                next.push((dest, !*current_signal));
                                // println!("{} -{}-> {}", key, signal_to_str(!*current_signal), dest);
                            }
                        } else {
                            let send_signal = !signals
                                .iter()
                                .map(|signal| states.get(signal).unwrap())
                                .all(|p| *p);
                            // for signal in signals.iter().map(||) {
                            //     let last_sent = states.get(signal).unwrap();
                            //     send_signal = send_signal && *last_sent;
                            // }
                            // for d in destinations.iter() {
                            // println!("{} -{}-> {}", key, signal_to_str(send_signal), d);
                            // }
                            next.append(
                                &mut destinations
                                    .iter()
                                    .map(|name| (*name, send_signal))
                                    .collect::<Vec<_>>(),
                            );
                        }
                    } else if let Some(state) = states.get_mut(key) {
                        if current_signal == &false {
                            *state = !*state;
                            if *state {
                                // for d in destinations.iter() {
                                //     println!("{} -high-> {}", key, d);
                                // }
                                next.append(
                                    &mut destinations
                                        .iter()
                                        .map(|name| (*name, true))
                                        .collect::<Vec<_>>(),
                                );
                            } else {
                                // for d in destinations.iter() {
                                //     println!("{} -low-> {}", key, d);
                                // }
                                next.append(
                                    &mut destinations
                                        .iter()
                                        .map(|name| (*name, false))
                                        .collect::<Vec<_>>(),
                                );
                            }
                        }
                    } else {
                        // for d in destinations.iter() {
                        //     println!("{} -{}-> {}", key, signal_to_str(*current_signal), d);
                        // }
                        next.append(
                            &mut destinations
                                .iter()
                                .map(|name| (*name, *current_signal))
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            for (_, signal) in &next {
                if *signal {
                    high_signals += 1;
                } else {
                    low_signals += 1;
                }
            }
            targets.clear();
            targets.append(&mut next);
        }
    }
    low_signals * high_signals
}
