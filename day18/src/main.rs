use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Num(isize),
    Op(char),
    BigBoi(Vec<Operation>),
}

impl Operation {
    fn operate(v: &mut Vec<Operation>, c: usize, opord: &[char]) {
        let op = v.remove(c);
        let mut n = v.remove(c);
        match op {
            Operation::Op('*') => {
                let x = v.get_mut(c - 1).unwrap();
                *x = Operation::Num(x.calc(opord) * n.calc(opord));
            }
            Operation::Op('+') => {
                let x = v.get_mut(c - 1).unwrap();
                *x = Operation::Num(x.calc(opord) + n.calc(opord));
            }
            _ => {}
        }
    }

    fn calc(&mut self, opord: &[char]) -> isize {
        match self {
            Operation::Op(_) => unreachable!(),
            Operation::Num(v) => *v,
            Operation::BigBoi(v) => {
                for &op in opord.iter() {
                    'op_loop: loop {
                        if let Some(c) = v.iter().position(|x| {
                            (matches!(x, Operation::Op(_)) && op == '_') || *x == Operation::Op(op)
                        }) {
                            Operation::operate(v, c, opord);
                        } else {
                            break 'op_loop;
                        }
                    }
                }
                v.pop().unwrap().calc(opord)
            }
        }
    }
}

fn solve_for_opord(opord: &[char]) -> isize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| fill_big_boi(&mut l.chars().filter(|x| !x.is_whitespace())).calc(opord))
        .sum::<isize>()
}
fn main() {
    println!("part1: {}", solve_for_opord(&['_']));
    println!("part2: {}", solve_for_opord(&['+', '*']));
}

fn fill_big_boi<I>(c: &mut I) -> Operation
where
    I: Iterator<Item = char>,
{
    let mut my_operators = Vec::new();
    'loep: while let Some(v) = c.next() {
        match v {
            '(' => {
                my_operators.push(fill_big_boi(c));
            }
            ')' => {
                break 'loep;
            }
            '*' | '+' => my_operators.push(Operation::Op(v)),
            '0'..='9' => my_operators.push(Operation::Num(v.to_digit(10).unwrap() as isize)),
            _ => {}
        };
    }
    if my_operators.len() == 1 {
        my_operators.pop().unwrap()
    } else {
        Operation::BigBoi(my_operators)
    }
}
