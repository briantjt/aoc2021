fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    let readings = contents
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let count = readings
        .windows(2)
        .filter(|a| a[1] > a[0])
        .count();
    println!("{}", count);

    let count2 = readings
        .windows(3)
        .map(|a| a.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|a| a[1] > a[0])
        .count();

    println!("{}", count2);
    Ok(())
}
