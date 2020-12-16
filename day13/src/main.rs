use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use num::Integer;

fn main() {
    let mut lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    let depart_time = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| !x.eq(&"x"))
        .map(|(i, x)| (i, x.parse::<usize>().unwrap()))
        .map(|(i, x)| ((x - i % x) % x, x))
        .collect::<Vec<(usize, usize)>>();
    part1(&depart_time, &busses);
    part2(&busses);
}

fn crt(n1: (usize, usize), n2: &(usize, usize)) -> (usize, usize) {
    for i in 0..n2.1 {
        let num = i * n1.1 + n1.0;
        if num % n2.1 == n2.0 {
            return (num, n2.1.lcm(&n1.1));
        }
    }
    panic!("I fucked up");
}

fn part2(busses: &[(usize, usize)]) {
    println!("part2: {:?}", busses.iter().fold((0, 1), crt).0);
}

fn part1(depart_time: &usize, busses: &[(usize, usize)]) {
    let (bus, delay) = busses
        .iter()
        .map(|x| (x.1, (x.1 - depart_time % x.1) % x.1))
        .min_by_key(|x| x.1)
        .unwrap();
    println!("part1: {}", bus * delay);
}
