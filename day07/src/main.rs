use std::{collections::HashMap, fs::File, io::prelude::*, io::BufReader};

use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let rules = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut x = l.split(" bags contain ");
            let bag = x.next().unwrap().to_string();
            let re = Regex::new(r"(\d+) (\S+ \S+) bags{0,1}").unwrap();
            (
                bag,
                re.captures_iter(&x.next().unwrap())
                    .map(|x| (x[2].to_string(), x[1].parse::<usize>().unwrap()))
                    .collect::<HashMap<String, usize>>(),
            )
        })
        .collect::<HashMap<String, HashMap<String, usize>>>();
    part1(&rules);
    part2(&rules);
}

fn part1(rules: &HashMap<String, HashMap<String, usize>>) {
    println!(
        "{}",
        rules
            .iter()
            .filter(|bag| can_contain(bag.0, "shiny gold", &rules))
            .count()
            - 1
    );
}

fn part2(rules: &HashMap<String, HashMap<String, usize>>) {
    println!("{}", total_bags("shiny gold", rules));
}

fn total_bags(s: &str, rules: &HashMap<String, HashMap<String, usize>>) -> usize {
    rules
        .get(s)
        .unwrap()
        .iter()
        .map(|b| b.1 + b.1 * total_bags(b.0, rules))
        .sum()
}

fn can_contain(s: &str, t: &str, rules: &HashMap<String, HashMap<String, usize>>) -> bool {
    s == t
        || rules
            .get(s)
            .unwrap()
            .iter()
            .any(|b| can_contain(b.0, t, rules))
}
