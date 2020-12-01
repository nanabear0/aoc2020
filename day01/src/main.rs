use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let numbers: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    let result: Option<i32> = numbers.iter().enumerate().find_map(|(i, n2)| {
        numbers.iter().enumerate().skip(i + 1).find_map(|(j, n1)| {
            numbers
                .iter()
                .skip(j + 1)
                .find(|x| **x == (2020 - n2 - n1))
                .map(|k| k * n1 * n2)
        })
    });
    println!("{:?}", result);
}
