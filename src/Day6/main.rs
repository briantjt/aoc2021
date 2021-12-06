fn count_fishes(fishes: &mut [u64; 9]) {
    let mut next_day_fishes = [0u64; 9];
    for day in 0..=8 {
        let new_fishes = fishes[day];
        if day == 0 {
            next_day_fishes[8] += new_fishes;
            next_day_fishes[6] += new_fishes;
        } else {
            next_day_fishes[day - 1] += new_fishes;
        }
    }
    *fishes = next_day_fishes;
}

fn main() -> std::io::Result<()> {
    let fishes: Vec<u8> = include_str!("input.txt")
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut num_fishes = [0u64; 9];
    for &f in fishes.iter() {
        num_fishes[f as usize] += 1;
    }
    for _ in 0..80 {
        count_fishes(&mut num_fishes);
    }
    let total_fishes: u64 = num_fishes.iter().sum();
    println!("{}", total_fishes);
    for _ in 80..256 {
        count_fishes(&mut num_fishes);
    }
    let total_fishes: u64 = num_fishes.iter().sum();
    println!("{}", total_fishes);
    Ok(())
}
