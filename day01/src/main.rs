use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let numbers: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    'l1: for (i, n1) in numbers.iter().enumerate() {
        for (j, n2) in numbers.iter().enumerate().skip(i + 1) {
            for (_, n3) in numbers.iter().enumerate().skip(j + 1) {
                if n1 + n2 + n3 == 2020 {
                    println!("{} {} {}, {}", n1, n2, n3, n1 * n2 * n3);
                    break 'l1;
                }
            }
        }
    }
}
