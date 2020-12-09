use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\S+) ([\-\+]\d+)$").unwrap();
        let cap = re.captures(s).unwrap();
        match cap[2].parse::<i32>() {
            Ok(n) => match &cap[1] {
                "acc" => Ok(Ins::Acc(n)),
                "jmp" => Ok(Ins::Jmp(n)),
                "nop" => Ok(Ins::Nop(n)),
                _ => Err("Invalid Op".to_owned()),
            },
            Err(_) => Err("Invalid Number".to_owned()),
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let instructions = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect::<Vec<Ins>>();

    part1(&instructions);
    part2(&instructions);
}

fn part2(instructions: &Vec<Ins>) {
    for (itc, _) in instructions.iter().enumerate().filter(|(_, x)| match x {
        Ins::Jmp(_) | Ins::Nop(_) => true,
        _ => false,
    }) {
        let (mut acc, mut pc) = (0, 0);
        let mut visited_instructions: HashSet<i32> = HashSet::new();

        'ml: loop {
            if visited_instructions.contains(&pc) {
                break 'ml;
            } else {
                visited_instructions.insert(pc);
            }
            if let Some(ins) = instructions.get(pc as usize) {
                match ins {
                    Ins::Acc(n) => {
                        acc += n;
                        pc += 1;
                    }
                    Ins::Jmp(n) => {
                        if pc != itc as i32 {
                            pc += n;
                        } else {
                            pc += 1;
                        }
                    }
                    Ins::Nop(n) => {
                        if pc != itc as i32 {
                            pc += 1;
                        } else {
                            pc += n;
                        }
                    }
                }
            } else {
                println!("part2: {}", acc);
                return;
            }
        }
    }
}

fn part1(instructions: &Vec<Ins>) {
    let (mut acc, mut pc) = (0, 0);
    let mut visited_instructions: HashSet<i32> = HashSet::new();

    'ml: loop {
        if visited_instructions.contains(&pc) {
            break 'ml;
        } else {
            visited_instructions.insert(pc);
        }
        if let Some(ins) = instructions.get(pc as usize) {
            match ins {
                Ins::Acc(n) => {
                    acc += n;
                    pc += 1;
                }
                Ins::Jmp(n) => {
                    pc += n;
                }
                Ins::Nop(_) => {
                    pc += 1;
                }
            }
        } else {
            break 'ml;
        }
    }
    println!("part1: {}", acc);
}
