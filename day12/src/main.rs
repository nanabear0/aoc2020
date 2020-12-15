use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let instructions = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let cap = Regex::new(r"^([A-Z]{1})(\d+)")
                .unwrap()
                .captures(&x)
                .unwrap();
            (
                cap[1].chars().next().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(char, i32)>>();
    part1(&instructions);
    part2(&instructions);
}

fn rotate_point(xi: &mut i32, yi: &mut i32, old_angle: &mut i32, angle: i32) {
    let rad_angle = (angle as f32).to_radians();
    let x = *xi as f32;
    let y = *yi as f32;
    *xi = (x * rad_angle.cos() - y * rad_angle.sin()).round() as i32;
    *yi = (x * rad_angle.sin() + y * rad_angle.cos()).round() as i32;
    *old_angle = (*old_angle + angle + 360) % 360;
}

fn part2(instructions: &[(char, i32)]) {
    let mut cur_dir = 0;
    let (mut wx, mut wy) = (10, 1);
    let (mut x, mut y) = (0, 0);
    for (dir, sp) in instructions {
        match dir {
            'N' => wy += sp,
            'S' => wy -= sp,
            'E' => wx += sp,
            'W' => wx -= sp,
            'L' => {
                rotate_point(&mut wx, &mut wy, &mut cur_dir, *sp);
            }
            'R' => {
                rotate_point(&mut wx, &mut wy, &mut cur_dir, -1 * sp);
            }
            'F' => {
                x += sp * wx;
                y += sp * wy;
            }
            _ => {}
        }
    }
    println!("part 2: {}", x.abs() + y.abs());
}

fn part1(instructions: &[(char, i32)]) {
    let mut cur_dir = 0;
    let (mut x, mut y) = (0, 0);
    for (dir, sp) in instructions {
        match dir {
            'N' => y += sp,
            'S' => y -= sp,
            'E' => x += sp,
            'W' => x -= sp,
            'L' => cur_dir = (cur_dir + sp + 360) % 360,
            'R' => cur_dir = (cur_dir - sp + 360) % 360,
            'F' => match cur_dir {
                0 => x += sp,
                90 => y += sp,
                180 => x -= sp,
                270 => y -= sp,
                _ => {}
            },
            _ => {}
        }
    }
    println!("part 1: {}", x.abs() + y.abs());
}
