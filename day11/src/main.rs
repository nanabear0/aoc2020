use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Space {
    Floor,
    Seat,
    Occupied,
    OutOfBounds,
}

static DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let mut seats = read_seat_map();
    loop {
        let mut changes = false;
        let new_seats = seats
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, y)| match y {
                        Space::Floor => Space::Floor,
                        Space::OutOfBounds => Space::OutOfBounds,
                        Space::Seat => {
                            if part2((i as i32, j as i32), &seats) == 0 {
                                changes = true;
                                Space::Occupied
                            } else {
                                Space::Seat
                            }
                        }
                        Space::Occupied => {
                            if part2((i as i32, j as i32), &seats) >= 5 {
                                changes = true;
                                Space::Seat
                            } else {
                                Space::Occupied
                            }
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Space>>>();
        if !changes {
            break;
        }
        // break;
        seats = new_seats;
    }
    println!(
        "part1: {:#?}",
        seats
            .iter()
            .map(|x| x.iter().filter(|&x| *x == Space::Occupied).count())
            .sum::<usize>()
    );
}

fn read_seat_map() -> Vec<Vec<Space>> {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    'L' => Space::Seat,
                    '.' => Space::Floor,
                    _ => Space::Occupied,
                })
                .collect()
        })
        .collect()
}

fn part2((i, j): (i32, i32), seats: &Vec<Vec<Space>>) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&&(x, y): &&(i32, i32)| {
            (1..100)
                .find_map(|n| {
                    let option = *seats
                        .get((i as i32 + x * n) as usize)
                        .map(|l| {
                            l.get((j as i32 + y * n) as usize)
                                .unwrap_or(&Space::OutOfBounds)
                        })
                        .unwrap_or(&Space::OutOfBounds);
                    if option != Space::Floor {
                        Some(option)
                    } else {
                        None
                    }
                })
                .unwrap()
                == Space::Occupied
        })
        .count()
}

fn _part1((i, j): (usize, usize), seats: &Vec<Vec<Space>>) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&&(x, y): &&(i32, i32)| {
            *seats
                .get((i as i32 + x) as usize)
                .map(|l| l.get((j as i32 + y) as usize).unwrap_or(&Space::Floor))
                .unwrap_or(&Space::Floor)
                == Space::Occupied
        })
        .count()
}
