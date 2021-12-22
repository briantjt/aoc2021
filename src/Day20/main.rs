fn convert_to_bin(s: &str) -> Option<u8> {
    match s {
        "." => Some(0),
        "#" => Some(1),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut contents = include_str!("input.txt").split("\n\n");
    let mapping: Vec<u8> = contents
        .next()
        .unwrap()
        .split("")
        .filter_map(convert_to_bin)
        .collect();
    let mut image: Vec<Vec<_>> = contents
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split("").filter_map(convert_to_bin).collect())
        .collect();
    // Index 0b000000000 and 0b111111111 are 0 and 1 respectively so the
    // infinite image alternates between light and dark for the out of bounds
    // parts. The puzzle asks for the number of light pixels after an even
    // number of iterations so it won't be infinity
    let mut out_of_bounds_color = 0;
    for i in 0..50 {
        let row_len = image.len() as i32;
        let col_len = image[0].len() as i32;
        // Extend the image by 2 rows and columns as the edge bits will be
        // neighbors of the extension
        let mut next_image = vec![vec![0; image[0].len() + 2]; image.len() + 2];
        for row_idx in 0..(next_image.len() as i32) {
            for col_idx in 0..(next_image[0].len() as i32) {
                let mut idx = 0usize;
                for row_d in [-1, 0, 1] {
                    for col_d in [-1, 0, 1] {
                        idx <<= 1;
                        let neighbor_row = row_idx + row_d - 1;
                        let neighbor_col = col_idx + col_d - 1;
                        if neighbor_row < 0
                            || neighbor_col < 0
                            || neighbor_row >= row_len
                            || neighbor_col >= col_len
                        {
                            idx += out_of_bounds_color;
                            continue;
                        }
                        idx += image[neighbor_row as usize][neighbor_col as usize] as usize;
                    }
                }
                next_image[row_idx as usize][col_idx as usize] = mapping[idx]
            }
        }
        out_of_bounds_color ^= 1;
        image = next_image;
        if i == 1 {
            let pixels_lit = image.iter().flatten().filter(|&&i| i == 1).count();
            println!("{}", pixels_lit);
        }
    }
    let pixels_lit = image.iter().flatten().filter(|&&i| i == 1).count();
    println!("{}", pixels_lit);
    Ok(())
}
