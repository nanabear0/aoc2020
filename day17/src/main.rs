use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn part2(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
    fn part1(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 0,
        }
    }
}

fn main() {
    let part = Point::part2;
    let re = BufReader::new(File::open("input.txt").unwrap());
    let mut map: HashSet<Point> = HashSet::new();
    re.lines().enumerate().for_each(|(y, line)| {
        line.unwrap()
            .chars()
            .enumerate()
            .filter(|c| c.1 == '#')
            .for_each(|(x, _)| {
                map.insert(Point {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                });
            })
    });
    for _ in 0..6 {
        let mut update_set: HashSet<Point> = HashSet::new();
        map.iter().for_each(|p| {
            NLIST.iter().for_each(|o| {
                update_set.insert(part(o, p));
            })
        });
        let mut new_map = HashSet::new();
        for s in update_set {
            let active = map.contains(&s);
            let neighbours = NLIST
                .iter()
                .skip(1)
                .filter(|o| map.contains(&part(&s, o)))
                .count();
            if neighbours == 3 || active && neighbours == 2 {
                new_map.insert(s);
            }
        }
        map = new_map;
    }
    println!("{}", map.len());
}

static NLIST: [Point; 81] = [
    Point {
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    },
    Point {
        w: 0,
        x: -1,
        y: -1,
        z: -1,
    },
    Point {
        w: 0,
        x: -1,
        y: -1,
        z: 0,
    },
    Point {
        w: 0,
        x: -1,
        y: -1,
        z: 1,
    },
    Point {
        w: 0,
        x: -1,
        y: 0,
        z: -1,
    },
    Point {
        w: 0,
        x: -1,
        y: 0,
        z: 0,
    },
    Point {
        w: 0,
        x: -1,
        y: 0,
        z: 1,
    },
    Point {
        w: 0,
        x: -1,
        y: 1,
        z: -1,
    },
    Point {
        w: 0,
        x: -1,
        y: 1,
        z: 0,
    },
    Point {
        w: 0,
        x: -1,
        y: 1,
        z: 1,
    },
    Point {
        w: 0,
        x: 0,
        y: -1,
        z: -1,
    },
    Point {
        w: 0,
        x: 0,
        y: -1,
        z: 0,
    },
    Point {
        w: 0,
        x: 0,
        y: -1,
        z: 1,
    },
    Point {
        w: 0,
        x: 0,
        y: 0,
        z: -1,
    },
    Point {
        w: 0,
        x: 0,
        y: 0,
        z: 1,
    },
    Point {
        w: 0,
        x: 0,
        y: 1,
        z: -1,
    },
    Point {
        w: 0,
        x: 0,
        y: 1,
        z: 0,
    },
    Point {
        w: 0,
        x: 0,
        y: 1,
        z: 1,
    },
    Point {
        w: 0,
        x: 1,
        y: -1,
        z: -1,
    },
    Point {
        w: 0,
        x: 1,
        y: -1,
        z: 0,
    },
    Point {
        w: 0,
        x: 1,
        y: -1,
        z: 1,
    },
    Point {
        w: 0,
        x: 1,
        y: 0,
        z: -1,
    },
    Point {
        w: 0,
        x: 1,
        y: 0,
        z: 0,
    },
    Point {
        w: 0,
        x: 1,
        y: 0,
        z: 1,
    },
    Point {
        w: 0,
        x: 1,
        y: 1,
        z: -1,
    },
    Point {
        w: 0,
        x: 1,
        y: 1,
        z: 0,
    },
    Point {
        w: 0,
        x: 1,
        y: 1,
        z: 1,
    },
    Point {
        w: -1,
        x: 0,
        y: 0,
        z: 0,
    },
    Point {
        w: -1,
        x: -1,
        y: -1,
        z: -1,
    },
    Point {
        w: -1,
        x: -1,
        y: -1,
        z: 0,
    },
    Point {
        w: -1,
        x: -1,
        y: -1,
        z: 1,
    },
    Point {
        w: -1,
        x: -1,
        y: 0,
        z: -1,
    },
    Point {
        w: -1,
        x: -1,
        y: 0,
        z: 0,
    },
    Point {
        w: -1,
        x: -1,
        y: 0,
        z: 1,
    },
    Point {
        w: -1,
        x: -1,
        y: 1,
        z: -1,
    },
    Point {
        w: -1,
        x: -1,
        y: 1,
        z: 0,
    },
    Point {
        w: -1,
        x: -1,
        y: 1,
        z: 1,
    },
    Point {
        w: -1,
        x: 0,
        y: -1,
        z: -1,
    },
    Point {
        w: -1,
        x: 0,
        y: -1,
        z: 0,
    },
    Point {
        w: -1,
        x: 0,
        y: -1,
        z: 1,
    },
    Point {
        w: -1,
        x: 0,
        y: 0,
        z: -1,
    },
    Point {
        w: -1,
        x: 0,
        y: 0,
        z: 1,
    },
    Point {
        w: -1,
        x: 0,
        y: 1,
        z: -1,
    },
    Point {
        w: -1,
        x: 0,
        y: 1,
        z: 0,
    },
    Point {
        w: -1,
        x: 0,
        y: 1,
        z: 1,
    },
    Point {
        w: -1,
        x: 1,
        y: -1,
        z: -1,
    },
    Point {
        w: -1,
        x: 1,
        y: -1,
        z: 0,
    },
    Point {
        w: -1,
        x: 1,
        y: -1,
        z: 1,
    },
    Point {
        w: -1,
        x: 1,
        y: 0,
        z: -1,
    },
    Point {
        w: -1,
        x: 1,
        y: 0,
        z: 0,
    },
    Point {
        w: -1,
        x: 1,
        y: 0,
        z: 1,
    },
    Point {
        w: -1,
        x: 1,
        y: 1,
        z: -1,
    },
    Point {
        w: -1,
        x: 1,
        y: 1,
        z: 0,
    },
    Point {
        w: -1,
        x: 1,
        y: 1,
        z: 1,
    },
    Point {
        w: 1,
        x: 0,
        y: 0,
        z: 0,
    },
    Point {
        w: 1,
        x: -1,
        y: -1,
        z: -1,
    },
    Point {
        w: 1,
        x: -1,
        y: -1,
        z: 0,
    },
    Point {
        w: 1,
        x: -1,
        y: -1,
        z: 1,
    },
    Point {
        w: 1,
        x: -1,
        y: 0,
        z: -1,
    },
    Point {
        w: 1,
        x: -1,
        y: 0,
        z: 0,
    },
    Point {
        w: 1,
        x: -1,
        y: 0,
        z: 1,
    },
    Point {
        w: 1,
        x: -1,
        y: 1,
        z: -1,
    },
    Point {
        w: 1,
        x: -1,
        y: 1,
        z: 0,
    },
    Point {
        w: 1,
        x: -1,
        y: 1,
        z: 1,
    },
    Point {
        w: 1,
        x: 0,
        y: -1,
        z: -1,
    },
    Point {
        w: 1,
        x: 0,
        y: -1,
        z: 0,
    },
    Point {
        w: 1,
        x: 0,
        y: -1,
        z: 1,
    },
    Point {
        w: 1,
        x: 0,
        y: 0,
        z: -1,
    },
    Point {
        w: 1,
        x: 0,
        y: 0,
        z: 1,
    },
    Point {
        w: 1,
        x: 0,
        y: 1,
        z: -1,
    },
    Point {
        w: 1,
        x: 0,
        y: 1,
        z: 0,
    },
    Point {
        w: 1,
        x: 0,
        y: 1,
        z: 1,
    },
    Point {
        w: 1,
        x: 1,
        y: -1,
        z: -1,
    },
    Point {
        w: 1,
        x: 1,
        y: -1,
        z: 0,
    },
    Point {
        w: 1,
        x: 1,
        y: -1,
        z: 1,
    },
    Point {
        w: 1,
        x: 1,
        y: 0,
        z: -1,
    },
    Point {
        w: 1,
        x: 1,
        y: 0,
        z: 0,
    },
    Point {
        w: 1,
        x: 1,
        y: 0,
        z: 1,
    },
    Point {
        w: 1,
        x: 1,
        y: 1,
        z: -1,
    },
    Point {
        w: 1,
        x: 1,
        y: 1,
        z: 0,
    },
    Point {
        w: 1,
        x: 1,
        y: 1,
        z: 1,
    },
];
