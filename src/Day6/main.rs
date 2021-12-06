const NEW_FISH_HATCH_TIME: usize = 9;
// New fishes get added 2 indexes (days) behind but it's easier to add positive integers
// for modulo so take the inverse modulo:
// (day-2) % 9 = (day % 9 + ((-2) % 9)) % 9 = (day % 9 + 7 % 9) % 9 = (day + 7) mod 9
const NEW_FISH_OFFSET: usize = 7;

fn main() -> std::io::Result<()> {
    let fishes: Vec<u8> = include_str!("input.txt")
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut num_fishes = [0u64; NEW_FISH_HATCH_TIME];
    for &f in fishes.iter() {
        num_fishes[f as usize] += 1;
    }
    for day in 0..80 {
        num_fishes[(day + NEW_FISH_OFFSET) % NEW_FISH_HATCH_TIME] +=
            num_fishes[day % NEW_FISH_HATCH_TIME];
    }
    let total_fishes: u64 = num_fishes.iter().sum();
    println!("{}", total_fishes);
    for day in 80..256 {
        num_fishes[(day + NEW_FISH_OFFSET) % NEW_FISH_HATCH_TIME] +=
            num_fishes[day % NEW_FISH_HATCH_TIME];
    }
    let total_fishes: u64 = num_fishes.iter().sum();
    println!("{}", total_fishes);
    Ok(())
}
