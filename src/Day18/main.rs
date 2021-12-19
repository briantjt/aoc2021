use std::{ops::Add, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]
struct Fish {
    values: Vec<u64>,
    depths: Vec<u8>,
}

#[derive(Debug)]
struct ParseFishErr;
impl FromStr for Fish {
    type Err = ParseFishErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        let mut depths = Vec::new();
        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {}
                d => {
                    values.push(d.to_digit(10).unwrap() as u64);
                    depths.push(depth);
                }
            }
        }
        Ok(Fish { values, depths })
    }
}

impl Fish {
    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self) -> bool {
        for i in 0..self.values.len() {
            if self.depths[i] != 5
                || i + 1 >= self.values.len()
                || self.depths[i] != self.depths[i + 1]
            {
                continue;
            }
            let left_value = self.values[i];
            let right_value = self.values[i + 1];
            self.values[i] = 0;
            self.depths[i] -= 1;
            if i != 0 {
                self.values[i - 1] += left_value;
            }
            if i + 2 != self.values.len() {
                self.values[i + 2] += right_value;
            }
            self.values.remove(i + 1);
            self.depths.remove(i + 1);
            return true;
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.values.len() {
            let value = self.values[i];
            if value < 10 {
                continue;
            }
            let (a, b) = {
                let half = value / 2;
                if value % 2 == 0 {
                    (half, half)
                } else {
                    (half, half + 1)
                }
            };
            self.values[i] = a;
            self.depths[i] += 1;
            self.values.insert(i + 1, b);
            self.depths.insert(i + 1, self.depths[i]);

            return true;
        }
        false
    }

    fn magnitude(&self) -> u64 {
        self.magnitude_recursive(0, 1).0
    }

    fn magnitude_recursive(&self, idx: usize, depth: u8) -> (u64, usize) {
        let (first_half, remainder) = if depth == self.depths[idx] {
            (self.values[idx], idx + 1)
        } else {
            self.magnitude_recursive(idx, depth + 1)
        };
        if idx == self.values.len() - 1 {
            return (first_half, remainder)
        }
        let (second_half, remainder) = if depth == self.depths[remainder] {
            (self.values[remainder], remainder + 1)
        } else {
            self.magnitude_recursive(remainder, depth + 1)
        };
        (3 * first_half + 2 * second_half, remainder)
    }
}

impl Add for Fish {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.values.extend(rhs.values);
        self.depths.extend(rhs.depths);
        self.depths.iter_mut().for_each(|d| *d += 1);
        self.reduce();
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let fish1: Fish = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let fish2: Fish = "[1,1]".parse().unwrap();
        assert_eq!(
            fish1 + fish2,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse::<Fish>().unwrap()
        );
    }

    #[test]
    fn test_magnitude_basic() {
        let fish: Fish = "[9,1]".parse().unwrap();
        assert_eq!(fish.magnitude(), 29);
    }

    #[test]
    fn test_nested_pair() {
        let fish: Fish = "[[9,1],[1,9]]".parse().unwrap();
        assert_eq!(fish.magnitude(), 129);
    }

    #[test]
    fn test_nested_outer() {
        let fish: Fish = "[[[9,1],1],3]".parse().unwrap();
        assert_eq!(fish.magnitude(), 273);
        let fish: Fish = "[3,[1,[9,1]]]".parse().unwrap();
        assert_eq!(fish.magnitude(), 131);
    }

    #[test]
    fn test_magnitude() {
        let fish: Fish = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse()
            .unwrap();
        assert_eq!(fish.magnitude(), 4140);
        let fish: Fish = "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]".parse().unwrap();
        assert_eq!(fish.magnitude(), 3993);
    }
}
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let snailfish: Vec<_> = contents
        .lines()
        .filter_map(|s| s.parse::<Fish>().ok())
        .collect();
    let solved = snailfish
        .clone()
        .into_iter()
        .reduce(|accum, next| accum + next)
        .unwrap();
    println!("{}", solved.magnitude());
    let mut highest_magnitude = 0;
    for x in 0..snailfish.len() {
        for y in 0..snailfish.len() {
            if x == y {
                continue;
            }
            let sum = snailfish[x].clone() + snailfish[y].clone();
            highest_magnitude = highest_magnitude.max(sum.magnitude());
        }
    }
    println!("{}", highest_magnitude);
    Ok(())
}
