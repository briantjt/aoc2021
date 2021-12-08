use std::collections::HashMap;

const UNIQUE_SEGMENT_COUNT: [u8; 4] = [2, 4, 3, 7]; // Digits 1, 4, 7, 8
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let mut signals: Vec<Vec<_>> = Vec::new();
    let mut segments: Vec<Vec<_>> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<_> = line.split(" | ").collect();
        signals.push(parts[0].split(' ').collect());
        segments.push(parts[1].split(' ').collect());
    }
    let unique_segment_count = segments
        .iter()
        .flatten()
        .filter(|seg| UNIQUE_SEGMENT_COUNT.contains(&(seg.len() as u8)))
        .count();
    println!("{}", unique_segment_count);

    let mut sum: u64 = 0;
    for (sequence_map, output) in signals
        .iter()
        .map(|v| map_sequence_to_digit(v))
        .zip(segments)
    {
        let mut number: u64 = 0;
        for s in output {
            let sorted_string = sort_string(s);
            let digit = sequence_map
                .get(&sorted_string)
                .expect("Couldn't find digit mapping");
            number *= 10;
            number += *digit as u64;
        }
        sum += number as u64;
    }
    println!("{}", sum);

    Ok(())
}

fn sort_string(s: &str) -> String {
    let mut sorted_seq: Vec<_> = s.chars().collect();
    sorted_seq.sort_unstable();
    String::from_iter(sorted_seq)
}

// Filter the 3 sequences of length 6 (0, 6, 9) to check for all chars of digit
// 7 - The one that does not have all chars of 7 will be the digit 6.  Compare the
// remaining 2 sequences of length 6 with 4 - The one that does not have all the
// chars of 4 will be 0. The other will be 9
//
// Compare the 3 sequences of length 5 (2, 3, 5) with 1 - The one that contains
// all chars of 1 will be 3. Compare remaining 2 sequences of
// length 5 (2, 5) with 6 - The one that is missing 2 chars will be 2 and the
// other will be 5
fn map_sequence_to_digit(seqs: &[&str]) -> HashMap<String, u8> {
    let mut length_to_seq = HashMap::new();
    for seq in seqs {
        let sorted_string = sort_string(seq);
        let length = sorted_string.len();
        length_to_seq
            .entry(length)
            .or_insert_with(Vec::new)
            .push(sorted_string);
    }
    let one = length_to_seq.get_mut(&2).unwrap().remove(0);
    let four = length_to_seq.get_mut(&4).unwrap().remove(0);
    let seven = length_to_seq.get_mut(&3).unwrap().remove(0);
    let eight = length_to_seq.get_mut(&7).unwrap().remove(0);

    let six_char_seqs = length_to_seq.get_mut(&6).unwrap();
    let six_pos = six_char_seqs
        .iter()
        .position(|seq| !seven.chars().all(|c| seq.contains(c)))
        .expect("Couldn't find digit 6");
    let six = six_char_seqs.remove(six_pos);
    let nine_pos = six_char_seqs
        .iter()
        .position(|seq| four.chars().all(|c| seq.contains(c)))
        .expect("Couldn't find digit 9");
    let nine = six_char_seqs.remove(nine_pos);
    let zero = six_char_seqs.remove(0);

    let five_char_seqs = length_to_seq.get_mut(&5).unwrap();
    let three_pos = five_char_seqs
        .iter()
        .position(|seq| one.chars().all(|c| seq.contains(c)))
        .expect("Couldnt find digit 3");
    let three = five_char_seqs.remove(three_pos);

    let two_pos = five_char_seqs
        .iter()
        .position(|seq| six.chars().filter(|c| !seq.contains(*c)).count() == 2)
        .expect("Couldn't find digit 2");
    let two = five_char_seqs.remove(two_pos);
    let five = five_char_seqs.remove(0);

    HashMap::from_iter(vec![
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ])
}
