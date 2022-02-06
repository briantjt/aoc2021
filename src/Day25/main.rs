use std::fmt;
#[derive(Clone, PartialEq, Eq, Copy)]
enum Floor {
    Right,
    Down,
    Empty,
}

impl fmt::Debug for Floor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Empty => write!(f, "."),
        }
    }
}

use itertools::Itertools;
use Floor::*;
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let mut sea_floor = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '>' => Right,
                    'v' => Down,
                    '.' => Empty,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let rows = sea_floor.len();
    let cols = sea_floor[0].len();
    let mut steps = 0;
    loop {
        let mut moved = 0;
        let mut row = 0;
        let mut col = 0;
        let mut next_sea_floor = vec![vec![Empty; cols]; rows];
        while row < rows {
            while col < cols {
                match sea_floor[row][col] {
                    Right => {
                        let next_col = (col + 1) % cols;
                        if sea_floor[row][next_col] == Empty {
                            moved += 1;
                            next_sea_floor[row][next_col] = Right;
                            next_sea_floor[row][col] = Empty;
                            col += 2;
                        } else {
                            next_sea_floor[row][col] = Right;
                            col += 1;
                        }
                    },
                    rest => {
                        next_sea_floor[row][col] = rest;
                        col += 1;
                    }
                }
            }
            col = 0;
            row += 1;
        }
        sea_floor = next_sea_floor;
        let mut row = 0;
        let mut col = 0;
        let mut next_sea_floor = vec![vec![Empty; cols]; rows];
        while col < cols {
            while row < rows {
                match sea_floor[row][col] {
                    Down => {
                        let next_row = (row + 1) % rows;
                        if sea_floor[next_row][col] == Empty {
                            moved += 1;
                            next_sea_floor[next_row][col] = Down;
                            next_sea_floor[row][col] = Empty;
                            row += 2;
                        } else {
                            next_sea_floor[row][col] = Down;
                            row += 1;
                        }
                    },
                    rest => {
                        next_sea_floor[row][col] = rest;
                        row += 1;
                    }
                }
            }
            row = 0;
            col += 1;
        }
        sea_floor = next_sea_floor;
        steps += 1;
        if moved == 0 {
            break
        }
    }
    println!("{}", steps);
    Ok(())
}
