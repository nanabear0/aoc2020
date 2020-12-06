use std::{collections::HashMap, collections::HashSet, fs::File, io::prelude::*, io::BufReader};

fn main() {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    let mut v = String::new();
    reader.read_to_string(&mut v).unwrap();
    part1(&v);
    part2(&v);
}

fn part2(v: &str) {
    println!(
        "{:?}",
        v.split("\r\n\r\n")
            .map(|x| {
                let mut total = 0;
                let mut a: HashMap<char, usize> = HashMap::new();
                x.split("\r\n").for_each(|f| {
                    f.chars().for_each(|c| *a.entry(c).or_insert(0) += 1);
                    total += 1;
                });
                a.iter().filter(|(_, &v)| v == total).count()
            })
            .sum::<usize>()
    );
}

fn part1(v: &str) {
    println!(
        "{:?}",
        v.split("\r\n\r\n")
            .map(|s| {
                s.chars()
                    .filter(char::is_ascii_alphanumeric)
                    .collect::<HashSet<char>>()
                    .len()
            })
            .sum::<usize>()
    );
}
