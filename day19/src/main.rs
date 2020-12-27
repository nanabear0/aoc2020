use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let mut char_rules: HashMap<char, usize> = HashMap::new();
    let mut rules: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    BufReader::new(File::open("rules.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .for_each(|l| {
            let cap = Regex::new(r"(\d+): (.*)").unwrap().captures(&l).unwrap();
            let r_num = cap[1].parse::<usize>().unwrap();
            let details = &cap[2];
            if details.contains('\"') {
                char_rules.insert(details.chars().nth(1).unwrap(), r_num);
            } else {
                rules.insert(
                    r_num,
                    details
                        .split(" | ")
                        .map(|x| {
                            Regex::new(r"(\d+)")
                                .unwrap()
                                .captures_iter(x)
                                .map(|c| c[1].parse::<usize>().unwrap())
                                .collect()
                        })
                        .collect(),
                );
            }
        });
    let lines: Vec<String> = BufReader::new(File::open("control.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();
    println!(
        "part1: {}",
        lines
            .iter()
            .filter(|l| {
                let mr: Vec<usize> = l.chars().map(|x| *char_rules.get(&x).unwrap()).collect();
                let v = check(&mr, 0, &rules, &char_rules.values().cloned().collect());
                *v.iter().max().unwrap_or(&0) == mr.len()
            })
            .count()
    );
}

fn check(
    mr: &[usize],
    rule: usize,
    rules: &HashMap<usize, Vec<Vec<usize>>>,
    char_rules: &HashSet<usize>,
) -> Vec<usize> {
    if char_rules.contains(&rule) {
        return if !mr.is_empty() && rule == mr[0] {
            vec![1]
        } else {
            vec![]
        };
    }
    let rule_set = rules.get(&rule).unwrap();
    let mut ret_val = vec![];
    for rxs in rule_set {
        let mut can_be_from = vec![0];
        for rx in rxs {
            let mut new_can_be = vec![];
            for i in can_be_from {
                check(&mr[i..], *rx, rules, char_rules)
                    .iter()
                    .for_each(|x| new_can_be.push(i + *x));
            }
            can_be_from = new_can_be;
        }
        ret_val.extend(can_be_from.iter());
    }
    ret_val
}
