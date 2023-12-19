use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
enum Condition {
    LessThan(usize),
    GreaterThan(usize),
}
#[derive(Debug)]
struct Rule {
    goto: String,
    subject: char,
    cond: Condition,
}

#[derive(Debug)]
struct Workflow {
    next: String,
    rules: Vec<Rule>,
}
const MAX: usize = 4000;
const MIN: usize = 0;

impl Workflow {
    fn branches(
        &mut self,
        instruction: HashMap<char, (usize, usize)>,
    ) -> Vec<(String, HashMap<char, (usize, usize)>)> {
        let mut end_instruction = instruction.clone();
        let mut branches: Vec<(String, HashMap<char, (usize, usize)>)> =
            Vec::with_capacity(self.rules.len());
        for rule in &self.rules {
            let mut new_instruction = end_instruction.clone();
            let goto = rule.goto.to_string();
            let (lower, upper) = end_instruction.get(&rule.subject).unwrap();
            match rule.cond {
                Condition::LessThan(value) => {
                    // upper
                    new_instruction.get_mut(&rule.subject).unwrap().1 = value.min(*upper);
                    end_instruction.get_mut(&rule.subject).unwrap().0 = value.max(*lower);
                }
                Condition::GreaterThan(value) => {
                    // lower
                    new_instruction.get_mut(&rule.subject).unwrap().0 = value.max(*lower);
                    end_instruction.get_mut(&rule.subject).unwrap().1 = value.min(*upper);
                }
            }
            branches.push((goto, new_instruction));
        }
        branches.push((self.next.to_string(), end_instruction));
        branches
    }

    fn run(&self, instruction: &HashMap<String, usize>) -> String {
        let mut result: Option<String> = None;
        for rule in &self.rules {
            match (&rule.cond, instruction.get(&rule.subject.to_string())) {
                (Condition::LessThan(value), Some(other)) => {
                    if value > other {
                        result = Some(rule.goto.clone());
                        break;
                    }
                }
                (Condition::GreaterThan(value), Some(other)) => {
                    if value < other {
                        result = Some(rule.goto.clone());
                        break;
                    }
                }
                (_, None) => {}
            }
        }
        result.unwrap_or(self.next.clone())
    }
}

fn main() {
    println!("puzzle one: {}", one(PUZZLE_INPUT));
    println!("puzzle two: {}", two(PUZZLE_INPUT));
}

fn one(puzzle_input: &str) -> usize {
    let mut reading_workflows = true;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut sum: usize = 0;
    for line in puzzle_input.lines() {
        if line.is_empty() {
            reading_workflows = false;
        } else if reading_workflows {
            let mut split = line.split(|ch| ch == '{' || ch == '}');
            let label = split.next().unwrap();
            let workflow = split.next().unwrap().split(",").collect::<Vec<_>>();
            let rules = workflow[0..workflow.len() - 1]
                .iter()
                .map(|rule| {
                    let (cond, goto) = rule.split_once(':').unwrap();
                    if let Some((category, remaining)) = cond.split_once('>') {
                        let number = remaining.parse::<usize>().unwrap();
                        let subject = category.parse::<char>().unwrap();
                        Rule {
                            goto: goto.to_string(),
                            subject,
                            cond: Condition::GreaterThan(number),
                        }
                    } else if let Some((category, remaining)) = cond.split_once('<') {
                        let number = remaining.parse::<usize>().unwrap();
                        let subject = category.parse::<char>().unwrap();
                        Rule {
                            goto: goto.to_string(),
                            subject,
                            cond: Condition::LessThan(number),
                        }
                    } else {
                        panic!();
                    }
                })
                .collect::<Vec<_>>();
            let next_workflow = workflow.last().unwrap();
            let workflow = Workflow {
                next: next_workflow.to_string(),
                rules,
            };
            workflows.insert(label.to_string(), workflow);
        } else {
            // parse the instructions
            let mut instruction = HashMap::with_capacity(4);
            for assignment in line[1..line.len() - 1].split(',') {
                let (lhs, rhs) = assignment.split_once('=').unwrap();
                instruction.insert(lhs.to_string(), rhs.parse::<usize>().unwrap());
            }
            let mut current = workflows.get("in").unwrap().run(&instruction);
            while current != "A" && current != "R" {
                current = workflows.get(&current).unwrap().run(&instruction);
            }
            if current == "A" {
                sum += instruction.iter().map(|(_key, value)| value).sum::<usize>();
            }
        }
    }
    sum
}

fn two(puzzle_input: &str) -> usize {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut combinations: usize = 0;
    for line in puzzle_input.lines() {
        if line.is_empty() {
            break;
        } else {
            let mut split = line.split(|ch| ch == '{' || ch == '}');
            let label = split.next().unwrap();
            let workflow = split.next().unwrap().split(",").collect::<Vec<_>>();
            let rules = workflow[0..workflow.len() - 1]
                .iter()
                .map(|rule| {
                    let (cond, goto) = rule.split_once(':').unwrap();
                    if let Some((category, remaining)) = cond.split_once('>') {
                        let number = remaining.parse::<usize>().unwrap();
                        let subject = category.parse::<char>().unwrap();
                        Rule {
                            goto: goto.to_string(),
                            subject,
                            cond: Condition::GreaterThan(number),
                        }
                    } else if let Some((category, remaining)) = cond.split_once('<') {
                        let number = remaining.parse::<usize>().unwrap();
                        let subject = category.parse::<char>().unwrap();
                        Rule {
                            goto: goto.to_string(),
                            subject,
                            cond: Condition::LessThan(number),
                        }
                    } else {
                        panic!();
                    }
                })
                .collect::<Vec<_>>();
            let next_workflow = workflow.last().unwrap();
            let workflow = Workflow {
                next: next_workflow.to_string(),
                rules,
            };
            workflows.insert(label.to_string(), workflow);
        }
    }
    let mut instruction: HashMap<char, (usize, usize)> = HashMap::new();
    instruction.insert('x', (MIN, MAX));
    instruction.insert('m', (MIN, MAX));
    instruction.insert('a', (MIN, MAX));
    instruction.insert('s', (MIN, MAX));
    let mut branches = workflows.get_mut("in").unwrap().branches(instruction);
    while branches.len() > 0 {
        let mut next_branches: Vec<(String, HashMap<char, (usize, usize)>)> = Vec::new();
        for (branch, branch_instruction) in branches {
            if branch == "R".to_string() {
            } else if branch == "A".to_string() {
                combinations += branch_instruction
                    .iter()
                    .fold(1, |acc, (_key, (min, max))| acc * min.abs_diff(*max));
            } else {
                let mut found_branches = workflows
                    .get_mut(&branch)
                    .unwrap()
                    .branches(branch_instruction);
                next_branches.append(&mut found_branches);
            }
        }
        branches = next_branches;
    }
    combinations
}
