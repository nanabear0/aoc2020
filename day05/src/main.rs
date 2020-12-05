use itertools::Itertools;
use std::{collections::BTreeSet, io::prelude::*};
use std::{fs::File, io::BufReader}; // 0.9.0

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let tickets = reader
        .lines()
        .map(|l| {
            usize::from_str_radix(
                &l.unwrap()
                    .chars()
                    .map(|c| match c {
                        'F' | 'L' => '0',
                        'B' | 'R' => '1',
                        _ => '0',
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .collect::<BTreeSet<usize>>();
    println!("part01: {:?}", tickets.iter().max());
    println!(
        "part02: {:?}",
        tickets.iter().tuple_windows().find_map(|(prev, next)| {
            if next - prev > 1 {
                Some(prev + 1)
            } else {
                None
            }
        })
    );
}
