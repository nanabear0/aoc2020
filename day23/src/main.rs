use std::{collections::VecDeque, time::SystemTime};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn main() {
    // let max = 9;
    let max = 1_000_000;
    let mut ring = "394618527"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<VecDeque<i32>>();
    (10..=max).into_iter().for_each(|x| ring.push_back(x));
    let mut sys_time = SystemTime::now();
    let mut dir = true;
    for i in 1..=10_000_000 {
        if i % 1_000 == 0 {
            let new_sys_time = SystemTime::now();
            println!(
                "{:8}: {:?}",
                i,
                new_sys_time.duration_since(sys_time).unwrap()
            );
            sys_time = new_sys_time;
        }
        let mut target = ring[0];
        ring.rotate_left(1);
        let x1 = ring.pop_front().unwrap();
        let x2 = ring.pop_front().unwrap();
        let x3 = ring.pop_front().unwrap();
        target = [-1, -2, -3, -4]
            .iter()
            .map(|&x| target + x)
            .find(|&x| x != x1 && x != x2 && x != x3)
            .unwrap();
        if target <= 0 {
            target = [0, -1, -2, -3]
                .iter()
                .map(|&x| max + x)
                .find(|&x| x != x1 && x != x2 && x != x3)
                .unwrap();
        }
        let mut it = ring.iter().enumerate();
        let pos;
        if dir {
            pos = it.rev().find(|(_, &x)| x == target).unwrap().0
        } else {
            pos = it.find(|(_, &x)| x == target).unwrap().0;
        }
        dir = pos as i32 > max / 2;
        ring.insert(pos + 1, x1);
        ring.insert(pos + 2, x2);
        ring.insert(pos + 3, x3);
        // ring.rotate_left(pos + 1);
        // ring.push_front(x3);
        // ring.push_front(x2);
        // ring.push_front(x1);
        // ring.rotate_right(pos + 1);
        // println!("{:?}", ring);
    }
    while ring[0] != 1 {
        ring.rotate_left(1);
    }
    println!(
        "part2: {} x {} = {:?}",
        ring[1],
        ring[2],
        ring[1] as i64 * ring[2] as i64
    );
}
