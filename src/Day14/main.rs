use std::collections::BTreeMap;
#[macro_use]
extern crate scan_fmt;

const FIRST_CHAR_MASK: u16 = 0xFF00;
const SECOND_CHAR_MASK: u16 = 0x00FF;

type PairMap = BTreeMap<u16, u8>;
type PairCount = BTreeMap<u16, u64>;
type CharCount = BTreeMap<u8, u64>;

fn main() -> std::io::Result<()> {
    let mut contents = include_str!("input.txt").split("\n\n");
    let template = contents.next().unwrap();
    let mut pair_count: PairCount = BTreeMap::new();
    let mut char_count: CharCount = BTreeMap::new();
    template
        .bytes()
        .for_each(|c| *char_count.entry(c).or_insert(0) += 1);
    template
        .bytes()
        .collect::<Vec<u8>>()
        .windows(2)
        .for_each(|pair| {
            *pair_count
                .entry(((pair[0] as u16) << 8) as u16 + pair[1] as u16)
                .or_insert(0) += 1
        });
    let pair_map: PairMap = contents
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            if let Ok((s, c)) = scan_fmt!(line, "{} -> {}", String, char) {
                let mut chars = s.bytes();
                let first = chars.next().unwrap() as u16;
                let second = chars.next().unwrap() as u16;
                Some(((first << 8) + second, c as u8))
            } else {
                None
            }
        })
        .collect();
    let build_next_chain = |(pair_count, mut char_count): (PairCount, CharCount)| {
        let mut next_pair_count = BTreeMap::new();
        for (pair, &count) in pair_count.iter() {
            let &element = pair_map.get(pair).unwrap();
            *next_pair_count
                .entry((pair & FIRST_CHAR_MASK) + element as u16)
                .or_insert(0) += count;
            *next_pair_count
                .entry(((element as u16) << 8) + (pair & SECOND_CHAR_MASK))
                .or_insert(0) += count;
            *char_count.entry(element).or_insert(0) += count;
        }
        (next_pair_count, char_count)
    };
    let mut counts = (pair_count, char_count);
    for _ in 0..10 {
        counts = build_next_chain(counts);
    }
    let max = counts.1.values().max().unwrap();
    let min = counts.1.values().min().unwrap();
    println!("{}", max - min);
    for _ in 10..40 {
        counts = build_next_chain(counts);
    }
    let max = counts.1.values().max().unwrap();
    let min = counts.1.values().min().unwrap();
    println!("{}", max - min);
    Ok(())
}
