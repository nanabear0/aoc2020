use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut p1 = parse_hand("input1.txt");
    let mut p2 = parse_hand("input2.txt");
    part1();
    println!("part2: {:?}", plej(&mut p1, &mut p2).1);
}

fn plej(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> (usize, usize) {
    let mut past_hands = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let new_hand = format!("{:?}{:?}", p1, p2);
        if past_hands.contains(&new_hand) {
            return (1, calculate(&p1));
        } else {
            past_hands.insert(new_hand);
        }
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        let winner;
        if p1.len() >= c1 && p2.len() >= c2 {
            let (winner_sub, _) = plej(
                &mut p1.iter().take(c1).cloned().collect(),
                &mut p2.iter().take(c2).cloned().collect(),
            );
            winner = winner_sub;
        } else {
            winner = if c1 > c2 { 1 } else { 2 };
        }
        if winner == 1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p1.is_empty() {
        (2, calculate(&p2))
    } else {
        (1, calculate(&p1))
    }
}

fn calculate(hand: &VecDeque<usize>) -> usize {
    hand.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x * (i + 1))
        .sum::<usize>()
}

fn part1() {
    let mut p1 = parse_hand("input1.txt");
    let mut p2 = parse_hand("input2.txt");
    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p1.is_empty() {
        p1 = p2;
    }
    println!(
        "part1: {}",
        p1.iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * (i + 1))
            .sum::<usize>()
    );
}

fn parse_hand(f: &str) -> VecDeque<usize> {
    BufReader::new(File::open(f).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
