#[macro_use]
extern crate scan_fmt;

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let (x1, x2, y1, y2) = scan_fmt!(
        contents,
        "target area: x={}..{}, y={}..{}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();
    // Has to be 1 less than the height: For e.g. initial velocity of 3
    // (3, 2), (5, 1), (6, 0), (6, -1), (5, -2), (3, -3), (0, -4), (-4, -5)
    let highest_y = ((-y1 - 1) * (-y1)) / 2;
    println!("{}", highest_y);
    let mut count = 0u64;
    for x in 0..=x2 {
        for y in y1..=(-y1) {
            let (mut x_vel, mut y_vel) = (x, y);
            let (mut x_pos, mut y_pos) = (0, 0);
            loop {
                x_pos += x_vel;
                y_pos += y_vel;
                if x_pos > x2 || y_pos < y1 {
                    break
                }
                if x1 <= x_pos && x_pos <= x2 && y1 <= y_pos && y_pos <= y2 {
                    count += 1;
                    break
                }
                x_vel = (x_vel -1).max(0);
                y_vel -= 1;
            }
        }
    }
    println!("{}", count);
    Ok(())
}
