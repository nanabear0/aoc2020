use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part2() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut chargers = reader
        .lines()
        .map(|s| s.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    chargers.sort();
    chargers.push(chargers.last().unwrap() + 3);
    let target = chargers.last().unwrap() + 3;
    let mut map = HashMap::new();
    println!("part2: {}", paths_from_to(0, target, &chargers, &mut map));
}

fn paths_from_to(
    start: usize,
    target: usize,
    chargers: &Vec<usize>,
    map: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(c) = map.get(&start) {
        *c
    } else {
        let s = chargers
            .iter()
            .filter(|c| (1..=3).any(|o| **c == o + start))
            .map(|x| {
                if *x + 3 == target {
                    1 as usize
                } else {
                    paths_from_to(*x, target, chargers, map)
                }
            })
            .sum();
        map.insert(start, s);
        s
    }
}

fn part1() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut chargers = reader
        .lines()
        .map(|s| s.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let my_charger = chargers.iter().max().unwrap() + 3;
    chargers.push(my_charger);
    chargers.push(0);
    chargers.sort();

    let (x1, x3) = chargers
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold((0, 0), |(x, y), z| match z {
            1 => (x + 1, y),
            3 => (x, y + 1),
            _ => (x, y),
        });
    println!("part1: {}", x1 * x3)
}
