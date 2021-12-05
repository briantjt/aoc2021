#[macro_use]
extern crate scan_fmt;
use std::str::FromStr;

struct Vent((i32, i32), (i32, i32));
#[derive(Debug, Clone)]
struct ParseVentErr;

impl FromStr for Vent {
    type Err = ParseVentErr;
    fn from_str(s: &str) -> Result<Vent, Self::Err> {
        let (x1, y1, x2, y2) =
            scan_fmt!(s, "{d},{d} -> {d},{d}", i32, i32, i32, i32).expect("Failed to parse input");
        Ok(Vent((x1, y1), (x2, y2)))
    }
}

type Map = [[u32; 1000]; 1000];
impl Vent {
    fn mark_map(&self, map: &mut Map) {
        let &Vent((mut x1, mut y1), (x2, y2)) = self;
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        loop {
            map[x1 as usize][y1 as usize] += 1;
            if (x1, y1) == (x2, y2) {
                break;
            }
            x1 += dx;
            y1 += dy;
        }
    }
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    let vents: Vec<Vent> = contents.filter_map(|line| line.parse().ok()).collect();
    let mut map = [[0; 1000]; 1000];
    vents
        .iter()
        .filter(|&&Vent((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|v| v.mark_map(&mut map));

    let most_dangerous_1 = map.iter().flatten().filter(|&&x| x >= 2).count();
    println!("{}", most_dangerous_1);

    vents
        .iter()
        .filter(|&&Vent((x1, y1), (x2, y2))| x1 != x2 && y1 != y2)
        .for_each(|v| v.mark_map(&mut map));
    let most_dangerous_2 = map.iter().flatten().filter(|&&x| x >= 2).count();
    println!("{}", most_dangerous_2);
    Ok(())
}
