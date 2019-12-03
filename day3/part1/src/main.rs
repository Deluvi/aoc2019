use std::collections::BTreeSet;
use std::io::{stdin, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

use Move::*;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Move {
    fn from_str(move_str: &str) -> Move {
        let amount = usize::from_str(
            move_str
                .get(1..)
                .expect("move_str[1..] should be a valid UTF-8 string"),
        )
        .expect("move_str[1..] should be a number");
        match move_str
            .get(0..1)
            .expect("move_str[0] should be a valid UTF-8 string")
        {
            "L" => Left(amount),
            "R" => Right(amount),
            "U" => Up(amount),
            "D" => Down(amount),
            _ => panic!("Unknown symbol"),
        }
    }
}

fn parse_moves_line(line: &str) -> Vec<Move> {
    line.split(',').map(Move::from_str).collect()
}

fn build_positions_set(moves: &[Move]) -> BTreeSet<Point> {
    let mut current_position = Point { x: 0, y: 0 };
    let mut point_set = BTreeSet::new();
    for move_inst in moves {
        match move_inst {
            Left(amount) => {
                for 
            }
        }
    }
    
    point_set
}

fn match

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let moves_wire1 = parse_moves_line(
        &lines
            .next()
            .expect("There should be a first line in the standard input")
            .unwrap(),
    );
    let moves_wire2 = parse_moves_line(
        &lines
            .next()
            .expect("There should be a second line in the standard input")
            .unwrap(),
    );
    println!("Wire 1: {:?}", moves_wire1);
    println!("Wire 2: {:?}", moves_wire2);
}
