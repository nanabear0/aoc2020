use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

use regex::Regex;

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
    matches: Vec<usize>,
    pos: Option<usize>,
}

impl Rule {
    fn check(&self, n: &usize) -> bool {
        self.range1.contains(n) || self.range2.contains(n)
    }
}

fn main() {
    let mut it = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    let mut rules: Vec<Rule> = vec![];
    'rules: loop {
        let s = it.next().unwrap();
        let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        if let Some(cap) = re.captures(&s) {
            rules.push(Rule {
                name: cap[1].to_owned(),
                range1: cap[2].parse::<usize>().unwrap()..=cap[3].parse::<usize>().unwrap(),
                range2: cap[4].parse::<usize>().unwrap()..=cap[5].parse::<usize>().unwrap(),
                matches: Vec::new(),
                pos: None,
            });
        } else {
            it.next();
            break 'rules;
        }
    }
    let your_ticket: Vec<usize> = it
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let total_fields = your_ticket.len();

    it.next();
    it.next();
    let nearby_tickets = it
        .map(|s| {
            s.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    println!(
        "part1: {}",
        nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .filter(|&field| !rules.iter().any(|rule| rule.check(field)))
            })
            .sum::<usize>()
    );

    let valid_nearby_tickets: Vec<Vec<usize>> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|field| rules.iter().any(|rule| rule.check(field)))
        })
        .cloned()
        .collect();
    for rule in &mut rules {
        for f in 0..total_fields {
            if valid_nearby_tickets
                .iter()
                .all(|ticket| rule.check(&ticket[f]))
            {
                rule.matches.push(f);
            }
        }
    }
    rules.sort_by_key(|rule| rule.matches.len());
    let mut already_matched = HashSet::new();
    for rule in &mut rules {
        let pos = *rule
            .matches
            .iter()
            .find(|&m| !already_matched.contains(m))
            .unwrap();
        already_matched.insert(pos);
        rule.pos = Some(pos);
    }

    println!(
        "part2: {}",
        rules
            .iter()
            .filter(|rule| rule.name.starts_with("departure"))
            .map(|rule| your_ticket[rule.pos.unwrap()])
            .product::<usize>()
    );
}
