use std::cmp::Ordering::*;

fn count_ones_and_zeroes(lines: &[Vec<char>], idx: usize) -> (u32, u32) {
    let mut num_ones = 0;
    let mut num_zeroes = 0;
    for line in lines.iter() {
        match line[idx] {
            '1' => num_ones += 1,
            '0' => num_zeroes += 1,
            _ => unreachable!(),
        }
    }
    (num_ones, num_zeroes)
}
fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").lines();
    let mut num_ones = [0u32; 12];
    let mut num_zeroes = [0u32; 12];
    for bin_str in contents.clone() {
        for (idx, c) in bin_str.chars().enumerate() {
            match c {
                '0' => num_zeroes[idx] += 1,
                '1' => num_ones[idx] += 1,
                _ => {}
            }
        }
    }
    let mut gamma = ['0'; 12];
    let mut epsilon = ['0'; 12];

    for (idx, (ones, zeroes)) in num_ones.into_iter().zip(num_zeroes).enumerate() {
        match ones.cmp(&zeroes) {
            Less | Equal => {
                epsilon[idx] = '1';
            }
            Greater => {
                gamma[idx] = '1';
            }
        }
    }
    let gamma_value =
        u32::from_str_radix(&String::from_iter(gamma), 2).expect("Failed to parse as integer");
    let epsilon_value =
        u32::from_str_radix(&String::from_iter(epsilon), 2).expect("Failed to parse as integer");
    println!("{}", gamma_value * epsilon_value);

    let mut idx = 0usize;
    let mut contents_oxygen = contents
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut contents_co2 = contents_oxygen.clone();
    loop {
        let (num_ones, num_zeroes) = count_ones_and_zeroes(&contents_oxygen, idx);
        contents_oxygen = contents_oxygen
            .into_iter()
            .filter(|line| match num_ones.cmp(&num_zeroes) {
                Greater | Equal => line[idx] == '1',
                Less => line[idx] == '0',
            })
            .collect();
        if contents_oxygen.len() == 1 {
            break;
        }
        idx += 1;
    }
    idx = 0;
    loop {
        let (num_ones, num_zeroes) = count_ones_and_zeroes(&contents_co2, idx);
        contents_co2 = contents_co2
            .into_iter()
            .filter(|line| match num_ones.cmp(&num_zeroes) {
                Greater | Equal => line[idx] == '0',
                Less => line[idx] == '1',
            })
            .collect();
        if contents_co2.len() <= 1 {
            break;
        }
        idx += 1;
    }
    let oxygen_value = u32::from_str_radix(&String::from_iter(contents_oxygen.remove(0)), 2)
        .expect("Failed to parse as integer");
    let co2_value = u32::from_str_radix(&String::from_iter(contents_co2.remove(0)), 2)
        .expect("Failed to parse as integer");
    println!("{}", oxygen_value * co2_value);

    Ok(())
}
