fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").trim();
    let mut positions: Vec<_> = contents
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    positions.sort_unstable();
    let median = {
        let total_crabs = positions.len();
        let middle_idx = total_crabs as f64 / 2f64;
        if total_crabs % 2 == 1 {
            positions[middle_idx as usize]
        } else {
            (positions[middle_idx.ceil() as usize] + positions[middle_idx.floor() as usize]) / 2
        }
    };
    let total_fuel: i64 = positions.iter().map(|p| (p-median).abs()).sum();
    println!("{}", total_fuel);
    let total_positions: i64 = positions.iter().sum();
    let average = total_positions / positions.len() as i64;
    let total_fuel_2: i64 = positions.iter().map(|p| {
        let n = (p-average).abs();
        (n * (n + 1)) / 2
    }).sum();
    println!("{}", total_fuel_2);

    Ok(())
}
