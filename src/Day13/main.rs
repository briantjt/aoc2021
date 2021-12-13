use std::cmp::max;

fn fold_paper(paper: Vec<Vec<bool>>, fold_along_y_axis: bool, center: usize) -> Vec<Vec<bool>> {
    let row_length = paper.len();
    let col_length = paper[0].len();
    let dots = paper
        .into_iter()
        .enumerate()
        .map(|(row, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(col, dot)| (row, col, dot))
        })
        .flatten();
    if fold_along_y_axis {
        let mut folded_paper = vec![vec![false; col_length]; center];
        for (row, col, dot) in dots {
            match row.cmp(&center) {
                std::cmp::Ordering::Less => folded_paper[row][col] = folded_paper[row][col] || dot,
                std::cmp::Ordering::Greater => {
                    let reflected_row = center - (row - center);
                    folded_paper[reflected_row][col] = folded_paper[reflected_row][col] || dot
                }
                std::cmp::Ordering::Equal => {}
            }
        }
        folded_paper
    } else {
        let mut folded_paper = vec![vec![false; center]; row_length];
        for (row, col, dot) in dots {
            match col.cmp(&center) {
                std::cmp::Ordering::Less => folded_paper[row][col] = folded_paper[row][col] || dot,
                std::cmp::Ordering::Greater => {
                    let reflected_col = center - (col - center);
                    folded_paper[row][reflected_col] = folded_paper[row][reflected_col] || dot
                }
                std::cmp::Ordering::Equal => {}
            }
        }
        folded_paper
    }
}

fn main() -> std::io::Result<()> {
    let mut contents = include_str!("input.txt").split("\n\n");
    let coords: Vec<Vec<usize>> = contents
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|coord| coord.parse().ok())
                .collect()
        })
        .collect();
    let fold_instructions: Vec<(bool, usize)> = contents
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            if let Some(part) = line.split(' ').nth(2) {
                let part: Vec<&str> = part.split('=').collect();
                // false for x axis, true for y axis
                let axis = part[0] == "y";
                let coord: usize = part[1].parse().unwrap();
                return (axis, coord);
            }
            unreachable!()
        })
        .collect();
    let mut max_col = 0;
    let mut max_row = 0;
    for coord in coords.iter() {
        max_col = max(max_col, coord[0]);
        max_row = max(max_row, coord[1]);
    }
    let mut paper = vec![vec![false; max_col + 1]; max_row + 1];
    for coord in coords.iter() {
        paper[coord[1]][coord[0]] = true;
    }
    let first_instruction = fold_instructions[0];
    paper = fold_paper(paper, first_instruction.0, first_instruction.1);
    println!(
        "{}",
        paper
            .iter()
            .map(|r| r.iter())
            .flatten()
            .filter(|&&dot| dot)
            .count()
    );
    for instruction in fold_instructions.iter().skip(1) {
        paper = fold_paper(paper, instruction.0, instruction.1);
    }
    for row in paper {
        for dot in row {
            print!("{}", if dot {"#"} else {"."});
        }
        println!();
    }
    Ok(())
}
