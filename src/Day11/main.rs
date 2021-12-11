use std::collections::VecDeque;

const CAVE_LENGTH: usize = 10;
const CAVE_HEIGHT: usize = 10;

fn is_out_of_bounds(row: usize, col: usize, row_d: usize, col_d: usize) -> bool {
    let row_out_of_bounds =
        (row == 0 && row_d == CAVE_HEIGHT - 1) || (row == CAVE_HEIGHT - 1 && row_d == 1);
    let col_out_of_bounds =
        (col == 0 && col_d == CAVE_LENGTH - 1) || (col == CAVE_LENGTH - 1 && col_d == 1);
    row_out_of_bounds || col_out_of_bounds
}

fn to_idx(row: usize, col: usize) -> usize {
    row * CAVE_LENGTH + col
}

fn to_row_and_col(idx: usize) -> (usize, usize) {
    (idx / 10, idx % 10)
}

fn main() -> std::io::Result<()> {
    let mut octopi = [0u8; 100];
    let mut total_flashes = 0;
    let mut has_flashed = [false; 100];
    let contents = include_str!("input.txt");
    contents
        .lines()
        .map(|line| line.split(""))
        .flatten()
        .filter_map(|s| s.parse().ok())
        .enumerate()
        .for_each(|(idx, level)| octopi[idx] = level);
    for step in 1.. {
        octopi.iter_mut().for_each(|o| *o += 1);
        let mut queue = VecDeque::new();
        for (idx, _) in octopi.iter().enumerate().filter(|&(_, &o)| o > 9) {
            total_flashes += 1;
            has_flashed[idx] = true;
            queue.push_back(idx);
        }
        while let Some(idx) = queue.pop_front() {
            for row_d in [CAVE_LENGTH - 1, 0, 1] {
                for col_d in [CAVE_HEIGHT - 1, 0, 1] {
                    if (row_d, col_d) == (0, 0) {
                        continue;
                    }
                    let (row, col) = to_row_and_col(idx);
                    if is_out_of_bounds(row, col, row_d, col_d) {
                        continue;
                    }
                    let adj_idx = to_idx((row + row_d) % CAVE_HEIGHT, (col + col_d) % CAVE_LENGTH);
                    octopi[adj_idx] += 1;
                    if octopi[adj_idx] > 9 && !has_flashed[adj_idx] {
                        total_flashes += 1;
                        has_flashed[adj_idx] = true;
                        queue.push_back(adj_idx);
                    }
                }
            }
        }
        octopi.iter_mut().for_each(|o| {
            if *o > 9 {
                *o = 0
            }
        });
        if step == 100 {
            println!("{}", total_flashes);
        }
        if has_flashed.iter().all(|b| *b) {
            println!("First sync: {}", step);
            break
        }
        has_flashed.iter_mut().for_each(|b| *b = false);
    }
    Ok(())
}
