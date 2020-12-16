use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    part1();
    part2();
}

fn add_memories(s: &str, n: usize, set: &mut HashSet<usize>) {
    if let Some(c) = s.chars().next() {
        match c {
            '0' => add_memories(&s[1..s.len()], n * 2, set),
            '1' => add_memories(&s[1..s.len()], (n + 1) * 2, set),
            _ => {
                add_memories(&s[1..s.len()], n * 2, set);
                add_memories(&s[1..s.len()], (n + 1) * 2, set);
            }
        };
    } else {
        set.insert(n);
    }
}

fn part2() {
    let re = BufReader::new(File::open("input.txt").unwrap());
    let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
    let mut mem = HashMap::new();
    for l in re.lines().map(Result::unwrap) {
        let regex = Regex::new(r"(mask = ([01X]+))|(mem\[(\d+)\] = (\d+))").unwrap();
        let captures = regex.captures(&l).unwrap();
        if let Some(mask) = captures.get(2) {
            current_mask = mask.as_str().to_owned();
        } else {
            let pos = captures[4].parse::<usize>().unwrap();
            let v = captures[5].parse::<usize>().unwrap();
            let mut new_mems = HashSet::new();
            add_memories(
                &format!("{:0current_mas.len()b}", pos)
                    .chars()
                    .zip(current_mask.chars())
                    .map(|(s, m)| match m {
                        'X' => 'X',
                        '0' => s,
                        '1' => '1',
                        _ => '0',
                    })
                    .collect::<String>(),
                0,
                &mut new_mems,
            );
            for x in new_mems {
                mem.insert(x, v);
            }
        }
    }
    println!("part2: {}", mem.values().sum::<usize>());
}

fn part1() {
    let re = BufReader::new(File::open("input.txt").unwrap());
    let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
    let mut mem = HashMap::new();
    for l in re.lines().map(Result::unwrap) {
        let regex = Regex::new(r"(mask = ([01X]+))|(mem\[(\d+)\] = (\d+))").unwrap();
        let captures = regex.captures(&l).unwrap();
        if let Some(mask) = captures.get(2) {
            current_mask = mask.as_str().to_owned();
        } else {
            let pos = captures[4].parse::<usize>().unwrap();
            let v = captures[5].parse::<usize>().unwrap();

            mem.insert(
                pos,
                usize::from_str_radix(
                    &format!("{:036b}", v)
                        .chars()
                        .zip(current_mask.chars())
                        .map(|(s, m)| match m {
                            'X' => s,
                            '0' => '0',
                            '1' => '1',
                            _ => '0',
                        })
                        .collect::<String>(),
                    2,
                )
                .unwrap(),
            );
        }
    }

    println!("part1: {}", mem.values().sum::<usize>());
}
