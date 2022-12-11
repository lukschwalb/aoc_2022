use std::fmt::{self};

struct Instruction {
    amount: u32,
    source: usize,
    target: usize,
}

#[derive(Copy, Clone, Debug)]
enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

impl fmt::Display for CraneModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut vector: Vec<Vec<char>> = Vec::new();

    for line in input.lines().rev() {
        if !line.contains("[") { continue; }

        for i in (1..line.len()).step_by(4) {
            let chars: Vec<char> = line.chars().collect();

            if vector.len() <= i/4 {
                vector.push(Vec::new());
            }
            match chars[i] {
                'A'..='Z' => vector[i/4].push(chars[i]),
                _ => (),
            }
        }
    }

    return vector;
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut vector: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        if !line.contains("move") { continue; }

        let space_split: Vec<&str> = line.split(" ").collect();
        if space_split.len() == 6 {
            let instruction = Instruction {
                amount: space_split[1].parse::<u32>().unwrap(),
                source: space_split[3].parse::<usize>().unwrap() - 1,
                target: space_split[5].parse::<usize>().unwrap() - 1,
            };

            vector.push(instruction);
        }
    }

    return vector;
}

fn execute_instructions(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>, model: CraneModel) {
    for instruction in instructions {
        match model {
            CraneModel::CrateMover9000 => {
                for _ in 0..instruction.amount {
                    if let Some(cra) = stacks[instruction.source].pop() {
                        stacks[instruction.target].push(cra);
                    }
                }
            },
            CraneModel::CrateMover9001 => {
                let stack_height = stacks[instruction.source].len();
                let remove_index: usize = stack_height - usize::try_from(instruction.amount).unwrap();

                for _ in 0..instruction.amount {
                    let removed = stacks[instruction.source].remove(remove_index);
                    stacks[instruction.target].push(removed);
                }
            },
        }
    }
}

fn main() {
    let input = include_str!("../aoc_input.txt");
    let instructions: Vec<Instruction> = parse_instructions(input);
    let models: [CraneModel; 2] = [CraneModel::CrateMover9000, CraneModel::CrateMover9001];

    for crane in models {
        let mut stacks: Vec<Vec<char>> = parse_crates(input);

        execute_instructions(&mut stacks, &instructions, crane);

        print!("Resulting top crates with {}: ", crane.to_string());
        for i in 0..stacks.len() {
            if let Some(last) = stacks[i].last() {
                print!("{}", last);
            }
        }
        print!("\n");
    }
}
