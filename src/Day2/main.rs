use std::str::FromStr;

enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug, Clone)]
struct ParseDirectionErr;

struct Submarine {
    horizontal_pos: i32,
    vertical_pos: i32,
    aim_pos: i32,
}

impl Default for Submarine {
    fn default() -> Self {
        Self {
            horizontal_pos: 0,
            vertical_pos: 0,
            aim_pos: 0,
        }
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionErr;
    fn from_str(s: &str) -> Result<Direction, ParseDirectionErr> {
        let dir_and_value: Vec<&str> = s.split(' ').collect();
        if dir_and_value.len() != 2 {
            return Err(ParseDirectionErr);
        }
        if let Ok(magnitude) = dir_and_value[1].parse::<i32>() {
            match dir_and_value[0] {
                "up" => Ok(Direction::Up(magnitude)),
                "down" => Ok(Direction::Down(magnitude)),
                "forward" => Ok(Direction::Forward(magnitude)),
                _ => Err(ParseDirectionErr),
            }
        } else {
            Err(ParseDirectionErr)
        }
    }
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    let directions: Vec<Direction> = contents
        .map(|line| line.parse::<Direction>().expect("Error parsing input"))
        .collect();

    let mut sub_part1 = Submarine::default();
    for dir in directions.iter() {
        match dir {
            Direction::Up(v) => sub_part1.vertical_pos -= v,
            Direction::Down(v) => sub_part1.vertical_pos += v,
            Direction::Forward(v) => sub_part1.horizontal_pos += v,
        };
    }
    println!("{}", sub_part1.horizontal_pos * sub_part1.vertical_pos);

    let mut sub_part2 = Submarine::default();
    for dir in directions.iter() {
        match dir {
            Direction::Up(v) => sub_part2.aim_pos -= v,
            Direction::Down(v) => sub_part2.aim_pos += v,
            Direction::Forward(v) => {
                sub_part2.horizontal_pos += v;
                sub_part2.vertical_pos += sub_part2.aim_pos * v;
            }
        };
    }
    println!("{}", sub_part2.horizontal_pos * sub_part2.vertical_pos);
    Ok(())
}
