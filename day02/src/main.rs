use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Password {
    r: usize,
    u: usize,
    c: char,
    s: String,
}

impl Password {
    fn p1(&self) -> bool {
        let count = self.s.chars().filter(|x| *x == self.c).count();
        count >= self.r && count <= self.u
    }
    fn p2(&self) -> bool {
        (self.s.chars().nth(self.r - 1).unwrap_or(' ') == self.c)
            ^ (self.s.chars().nth(self.u - 1).unwrap_or(' ') == self.c)
    }
}

impl FromStr for Password {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Password {
            r: caps[1].parse().unwrap(),
            u: caps[2].parse().unwrap(),
            c: caps[3].chars().next().unwrap(),
            s: caps[4].to_string(),
        })
    }
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let passwords: Vec<Password> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();
    println!("p1: {:?}", passwords.iter().filter(|p| p.p1()).count());
    println!("p2: {:?}", passwords.iter().filter(|p| p.p2()).count());
}
