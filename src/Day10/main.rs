use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Bracket {
    LRound = 0,
    LSquare,
    LCurly,
    LAngle,
    RRound,
    RSquare,
    RCurly,
    RAngle,
}
const LEFT_BRACKET_OFFSET: u8 = 4;

use self::Bracket::*;

struct ParseBracketError;

impl FromStr for Bracket {
    type Err = ParseBracketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bracket = match s {
            "[" => LSquare,
            "]" => RSquare,
            "<" => LAngle,
            ">" => RAngle,
            "(" => LRound,
            ")" => RRound,
            "{" => LCurly,
            "}" => RCurly,
            _ => return Err(ParseBracketError),
        };
        Ok(bracket)
    }
}

type Col = usize;
enum ParseError {
    Incomplete(Vec<Bracket>),
    Invalid(Col, Bracket),
}

fn parse_line(brackets: &[Bracket]) -> ParseError {
    let mut stack = Vec::new();
    for (col, &bracket) in (1..).zip(brackets.iter()) {
        match bracket {
            LSquare | LAngle | LRound | LCurly => {
                stack.push(bracket);
            }
            _ => match stack.pop() {
                Some(opening_bracket) => {
                    if bracket as u8 - LEFT_BRACKET_OFFSET != opening_bracket as u8 {
                        return ParseError::Invalid(col, bracket);
                    }
                }
                None => return ParseError::Invalid(col, bracket),
            },
        }
    }
    ParseError::Incomplete(stack)
}
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    let lines: Vec<Vec<Bracket>> = contents
        .map(|line| line.split("").filter_map(|s| s.parse().ok()).collect())
        .collect();
    let parsed_lines: Vec<ParseError> = lines.iter().map(|line| parse_line(line)).collect();
    let score: u64 = parsed_lines
        .iter()
        .enumerate()
        .map(|(_, err)| match err {
            ParseError::Invalid(_, b) => match b {
                RRound => 3,
                RSquare => 57,
                RCurly => 1197,
                RAngle => 25137,
                _ => 0,
            },
            _ => 0,
        })
        .sum();
    println!("{}", score);
    let mut completed_scores: Vec<u64> = parsed_lines
        .iter()
        .filter_map(|err| match err {
            ParseError::Incomplete(brackets) => {
                let score = brackets.iter().rev().fold(0, |score, b| {
                    let next_score = score * 5;
                    match b {
                        LRound => next_score + 1,
                        LSquare => next_score + 2,
                        LCurly => next_score + 3,
                        LAngle => next_score + 4,
                        _ => unreachable!(),
                    }
                });
                Some(score)
            }
            ParseError::Invalid(_, _) => None,
        })
        .collect();
    completed_scores.sort_unstable();
    println!("{}", completed_scores[completed_scores.len() / 2]);
    Ok(())
}
