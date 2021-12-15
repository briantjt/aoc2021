use aoc2021::is_out_of_bounds;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(cave: &[Vec<u8>]) -> u32 {
    let row_length = cave.len();
    let col_length = cave[0].len();
    let goal = (row_length - 1, col_length - 1);
    let mut dist: Vec<Vec<u32>> = (0..cave.len())
        .map(|_| (0..cave[0].len()).map(|_| u32::MAX).collect())
        .collect();
    dist[0][0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0)));
    let deltas = [(1, 0), (row_length - 1, 0), (0, 1), (0, col_length - 1)];
    while let Some(Reverse((cost, row, col))) = heap.pop() {
        if (row, col) == goal {
            return cost;
        }
        if cost > dist[row][col] {
            continue;
        }
        for (row_d, col_d) in deltas {
            if is_out_of_bounds(row, col, row_d, col_d, row_length, col_length) {
                continue;
            }
            let next_row = (row + row_d) % row_length;
            let next_col = (col + col_d) % col_length;
            let next_cost = cost + (cave[next_row][next_col] as u32);
            if next_cost < dist[next_row][next_col] {
                heap.push(Reverse((next_cost, next_row, next_col)));
                dist[next_row][next_col] = next_cost;
            }
        }
    }
    unreachable!();
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let cave: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|c| c.parse::<u8>().ok())
                .collect()
        })
        .collect();
    let cost = dijkstra(&cave);
    println!("{}", cost);
    let cave_rows = cave.len();
    let cave_cols = cave[0].len();
    let actual_cave: Vec<Vec<_>> = (0..(5 * cave_rows)).map(|row| {
        (0..(5 * cave_cols)).map(|col| {
            let row_level = row / cave_rows;
            let col_level = col / cave_cols;
            let cost = cave[row % cave_rows][col % cave_cols] + row_level as u8 + col_level as u8;
            if cost > 9 {
                cost - 9
            } else {
                cost
            }
        }).collect()
    }).collect();
    let cost = dijkstra(&actual_cave);
    println!("{}", cost);
    Ok(())
}
