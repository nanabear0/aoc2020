use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    if let Some(target) = part1() {
        part2(target);
    }
}

fn part2(target: usize) {
    let mut buffer = VecDeque::new();
    let mut buffer_sum = 0 as usize;
    'l: for s in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<usize>().unwrap())
    {
        buffer.push_front(s);
        buffer_sum += s;

        if buffer_sum == target && buffer.len() >= 2 {
            break 'l;
        } else {
            while buffer_sum > target && !buffer.is_empty() {
                buffer_sum -= buffer.pop_back().unwrap();
                if buffer_sum == target && buffer.len() >= 2 {
                    break 'l;
                }
            }
        }
    }
    if buffer.len() >= 2 {
        println!(
            "part2: {}",
            buffer.iter().min().unwrap() + buffer.iter().max().unwrap()
        );
    } else {
        println!("part2 borken");
    }
}

fn part1() -> Option<usize> {
    let mut buffer = VecDeque::new();
    for s in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<usize>().unwrap())
    {
        if buffer.len() < 25 {
            buffer.push_front(s);
        } else if buffer
            .iter()
            .enumerate()
            .any(|(i, x)| buffer.iter().skip(i).any(|y| x + y == s))
        {
            buffer.push_front(s);
            buffer.pop_back();
        } else {
            println!("part1: {}", s);
            return Option::Some(s);
        }
    }
    println!("part1 borken");
    Option::None
}
