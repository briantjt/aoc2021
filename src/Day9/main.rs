use std::collections::{HashSet, VecDeque};

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    let floor: Vec<Vec<u8>> = contents
        .map(|line| {
            line.split("")
                .filter_map(|c| c.parse::<u8>().ok())
                .collect()
        })
        .collect();
    let num_rows = floor.len();
    let num_cols = floor[0].len();
    let is_out_of_bounds = |row, col, row_d, col_d| {
        let out_of_row_bounds =
            (row_d == num_rows - 1 && row == 0) || (row_d == 1 && row == num_rows - 1);
        let out_of_col_bounds =
            (col_d == num_cols - 1 && col == 0) || (col_d == 1 && col == num_cols - 1);
        out_of_row_bounds || out_of_col_bounds
    };
    let delta: [(usize, usize); 4] = [(num_rows - 1, 0), (1, 0), (0, num_cols - 1), (0, 1)];
    let lowest_floors: Vec<(usize, usize, u8)> = floor
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, &height)| {
                    let is_lowest = delta.iter().all(|&(row_d, col_d)| {
                        is_out_of_bounds(row, col, row_d, col_d)
                            || height < floor[(row + row_d) % num_rows][(col + col_d) % num_cols]
                    });
                    if is_lowest {
                        Some((row, col, height))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize, u8)>>()
        })
        .flatten()
        .collect();
    println!(
        "{}",
        lowest_floors
            .iter()
            .map(|&(_, _, height)| height as u64 + 1)
            .sum::<u64>()
    );
    let mut basins: Vec<usize> = Vec::with_capacity(lowest_floors.len());
    for (row, col, height) in lowest_floors.into_iter() {
        let mut size = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize, u8)> = VecDeque::new();
        queue.push_back((row, col, height));
        while let Some((row, col, height)) = queue.pop_front() {
            if visited.contains(&(row, col)) {
                continue;
            }
            visited.insert((row, col));
            size += 1;
            for &(row_d, col_d) in delta.iter() {
                if is_out_of_bounds(row, col, row_d, col_d) {
                    continue;
                }
                let next_row = (row + row_d) % num_rows;
                let next_col = (col + col_d) % num_cols;
                let next_height = floor[next_row][next_col];
                if next_height < 9 && next_height > height {
                    queue.push_back((next_row, next_col, next_height));
                }
            }
        }
        basins.push(size);
    }
    basins.sort_unstable();
    println!("{}", basins.iter().rev().take(3).product::<usize>());
    Ok(())
}
