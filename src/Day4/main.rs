use std::str::FromStr;

struct BingoBoard([u8; 25]);

#[derive(Debug, Clone)]
struct ParseBingoBoardError;

impl FromStr for BingoBoard {
    type Err = ParseBingoBoardError;
    fn from_str(nums: &str) -> Result<Self, Self::Err> {
        let mut board = [0u8; 25];
        for (num, pos) in nums
            .split(&['\n', ' '][..])
            .filter(|&s| !s.is_empty())
            .zip(board.iter_mut())
        {
            match num.parse::<u8>() {
                Ok(v) => *pos = v,
                Err(_) => return Err(ParseBingoBoardError),
            }
        }
        Ok(Self(board))
    }
}

impl BingoBoard {
    const VERTICAL_INDICES: [[usize; 5]; 5] = [
        [0, 5, 10, 15, 20],
        [1, 6, 11, 16, 21],
        [2, 7, 12, 17, 22],
        [3, 8, 13, 18, 23],
        [4, 9, 14, 19, 24],
    ];

    fn mark_num(&mut self, num: u8) {
        for pos in self.0.iter_mut() {
            if *pos == num {
                *pos = 0;
            }
        }
    }

    fn has_bingo(&self) -> bool {
        let horizontal_check = self.0.chunks(5).any(|row| row.iter().all(|&i| i == 0));
        let vertical_check = Self::VERTICAL_INDICES
            .iter()
            .any(|indices| indices.iter().all(|&i| self.0[i] == 0));
        horizontal_check || vertical_check
    }

    fn count_score(&self, num: u8) -> u32 {
        let board_sum: u32 = self.0.iter().map(|&i| i as u32).sum();
        board_sum * num as u32
    }
}

fn main() -> std::io::Result<()> {
    let mut contents: Vec<_> = include_str!("input.txt").trim().split("\n\n").collect();
    let chosen_nums: Vec<_> = contents
        .remove(0)
        .split(',')
        .map(|s| s.parse::<u8>().expect("Failed to parse as integer"))
        .collect::<Vec<_>>();
    let mut bingo_boards = contents
        .into_iter()
        .map(BingoBoard::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse board");
    let mut scores = Vec::new();
    for num in chosen_nums {
        for board in bingo_boards.iter_mut() {
            board.mark_num(num);
        }
        bingo_boards = bingo_boards
            .into_iter()
            .filter(|board| {
                let has_bingo = board.has_bingo();
                if has_bingo {
                    scores.push(board.count_score(num));
                }
                !has_bingo
            })
            .collect();
    }
    println!("{:?}", scores);
    println!("{}", scores.first().unwrap());
    println!("{}", scores.last().unwrap());
    Ok(())
}
