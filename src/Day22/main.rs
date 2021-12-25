use scan_fmt::scan_fmt;

struct Coords(i32, i32);

struct Cuboid {
    x: Coords,
    y: Coords,
    z: Coords,
    state: bool,
}

impl Coords {
    fn intersection(&self, other: &Self) -> Option<Self> {
        if other.0 > self.1 || self.0 > other.1 {
            None
        } else {
            Some(Self(self.0.max(other.0), self.1.min(other.1)))
        }
    }
}

impl Cuboid {
    fn new(state: bool, coords: (i32, i32, i32, i32, i32, i32)) -> Self {
        let (x1, x2, y1, y2, z1, z2) = coords;
        Cuboid {
            x: Coords(x1, x2),
            y: Coords(y1, y2),
            z: Coords(z1, z2),
            state,
        }
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        self.x.intersection(&other.x).and_then(|x| {
            self.y.intersection(&other.y).and_then(|y| {
                self.z.intersection(&other.z).map(|z| Self {
                    x,
                    y,
                    z,
                    state: !self.state,
                })
            })
        })
    }
    // Add one because it's made of cubes - it stretches out by 1 unit more at
    // the end
    fn volume(&self) -> i64 {
        let vol = (self.x.1 - self.x.0 + 1) as i64
            * (self.y.1 - self.y.0 + 1) as i64
            * (self.z.1 - self.z.0 + 1) as i64;
        if self.state {
            vol
        } else {
            -vol
        }
    }
}
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let instructions: Vec<_> = contents
        .lines()
        .filter_map(|line| {
            let formats = [
                (true, "on x={}..{},y={}..{},z={}..{}"),
                (false, "off x={}..{},y={}..{},z={}..{}"),
            ];
            for (state, format) in formats {
                if let Ok(result) = scan_fmt!(line, format, i32, i32, i32, i32, i32, i32) {
                    return Some((state, result));
                }
            }
            None
        })
        .collect();
    let mut cubes = [[[false; 101]; 101]; 101];
    for &(state, (x1, x2, y1, y2, z1, z2)) in instructions.iter().take(20) {
        for i in (x1 + 50)..=(x2 + 50) {
            for j in (y1 + 50)..=(y2 + 50) {
                for z in (z1 + 50)..=(z2 + 50) {
                    cubes[i as usize][j as usize][z as usize] = state;
                }
            }
        }
    }
    let on_cubes = cubes.iter().flatten().flatten().filter(|&&x| x).count();
    println!("{}", on_cubes);
    let cuboids = instructions
        .into_iter()
        .map(|(state, coords)| Cuboid::new(state, coords))
        .fold(Vec::new(), |mut cuboids: Vec<Cuboid>, step| {
            cuboids.extend(
                cuboids
                    .iter()
                    .filter_map(|c| c.intersection(&step))
                    .collect::<Vec<_>>(),
            );
            if step.state {
                cuboids.push(step)
            }
            cuboids
        });
    let total = cuboids.into_iter().map(|c| c.volume()).sum::<i64>();
    println!("{}", total);
    Ok(())
}
