use std::collections::BTreeMap;
#[macro_use]
extern crate scan_fmt;
fn main() -> std::io::Result<()> {
    let mut contents = include_str!("input.txt").split("\n\n");
    let template = contents.next().unwrap();
    let mut pair_count: BTreeMap<[u8; 2], u64> = BTreeMap::new();
    let mut char_count: BTreeMap<u8, u64> = BTreeMap::new();
    template
        .bytes()
        .for_each(|c| *char_count.entry(c).or_insert(0) += 1);
    template
        .bytes()
        .collect::<Vec<u8>>()
        .windows(2)
        .for_each(|pair| *pair_count.entry([pair[0], pair[1]]).or_insert(0) += 1);
    let pair_map: BTreeMap<[u8; 2], u8> = contents
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            if let Ok((s, c)) = scan_fmt!(line, "{} -> {}",  String, char) {
                let mut chars = s.bytes();
                Some((
                    [chars.next().unwrap(), chars.next().unwrap()],
                    c as u8,
                ))
            } else {
                None
            }
        })
        .collect();
    let build_next_chain = |pair_count: BTreeMap<[u8; 2], u64>, mut char_count: BTreeMap<u8, u64>| {
        let mut next_pair_count = BTreeMap::new();
        for (pair, &count) in pair_count.iter() {
            let &element = pair_map.get(pair).unwrap();
            *next_pair_count.entry([pair[0], element]).or_insert(0) += count;
            *next_pair_count.entry([element, pair[1]]).or_insert(0) += count;
            *char_count.entry(element).or_insert(0) += count;
        }
        (next_pair_count, char_count)
    };
    let mut counts = (pair_count, char_count);
    for _ in 0..10 {
        counts = build_next_chain(counts.0, counts.1);
    }
    let max = counts.1.values().max().unwrap();
    let min = counts.1.values().min().unwrap();
    println!("{}", max - min);
    for _ in 10..40 {
        counts = build_next_chain(counts.0, counts.1);
    }
    let max = counts.1.values().max().unwrap();
    let min = counts.1.values().min().unwrap();
    println!("{}", max - min);
    Ok(())
}
