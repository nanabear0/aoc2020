use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

static NLIST: [&str; 7] = ["own", "e", "w", "ne", "nw", "se", "sw"];

fn main() {
    let mut black_tiles = BTreeSet::new();
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|x| {
            Regex::new(r"[ns]?[ew]")
                .unwrap()
                .captures_iter(&x)
                .fold((0, 0), |(ix, iy), d| {
                    let (nx, ny) = calc_offset(&iy, &d[0]);
                    (ix + nx, iy + ny)
                })
        })
        .for_each(|x| {
            if black_tiles.contains(&x) {
                black_tiles.remove(&x);
            } else {
                black_tiles.insert(x);
            }
        });
    println!("part1: {}", black_tiles.len());

    for _ in 1..=100 {
        let mut update_set: BTreeSet<(isize, isize)> = BTreeSet::new();
        black_tiles.iter().for_each(|(px, py)| {
            NLIST.iter().for_each(|o| {
                let (nx, ny) = calc_offset(py, *o);
                update_set.insert((px + nx, ny + py));
            })
        });
        let mut new_tiles = BTreeSet::new();
        for (px, py) in update_set {
            let is_black = black_tiles.contains(&(px, py));
            let black_neighbours = NLIST
                .iter()
                .skip(1)
                .filter(|o| {
                    let (nx, ny) = calc_offset(&py, **o);
                    black_tiles.contains(&(px + nx, ny + py))
                })
                .count();
            if black_neighbours == 2 || is_black && black_neighbours == 1 {
                new_tiles.insert((px, py));
            }
        }
        black_tiles = new_tiles;
    }
    println!("part2: {}", black_tiles.len());
}

fn calc_offset(cur_y: &isize, dir: &str) -> (isize, isize) {
    let (edp, wdp) = if *cur_y % 2 == 0 { (1, 0) } else { (0, -1) };
    match dir {
        "e" => (1, 0),
        "w" => (-1, 0),
        "ne" => (edp, 1),
        "nw" => (wdp, 1),
        "se" => (edp, -1),
        "sw" => (wdp, -1),
        "own" => (0, 0),
        _ => unreachable!(),
    }
}
