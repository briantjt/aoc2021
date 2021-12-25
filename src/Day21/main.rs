use std::collections::HashMap;

use itertools::iproduct;

use scan_fmt::scan_fmt;

#[derive(Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Player {
    pos: u32,
    score: u32
}

impl Player {
    fn play(&mut self, roll: u32) {
        self.pos = (self.pos + roll - 1) % 10 + 1;
        self.score += self.pos
    }
}
fn main() -> std::io::Result<()> {
    let mut contents = include_str!("input.txt").lines();
    let p1 = contents
        .next()
        .map(|line| scan_fmt!(line, "Player 1 starting position: {}", u32).unwrap())
        .unwrap();
    let p2 = contents
        .next()
        .map(|line| scan_fmt!(line, "Player 2 starting position: {}", u32).unwrap())
        .unwrap();
    let mut player_1 = p1;
    let mut player_2 = p2;
    let (mut player_1_score, mut player_2_score) = (0u32, 0);
    let mut die = 1;
    let mut dice_rolls = 0;
    'game: loop {
        for (i, player, score) in [
            (1, &mut player_1, &mut player_1_score),
            (2, &mut player_2, &mut player_2_score),
        ] {
            let mut roll = 0;
            for _ in 0..3 {
                roll += die;
                die = die % 100 + 1;
            }
            dice_rolls += 3;
            *player = {
                let space = (*player + roll) % 10;
                if space == 0 {
                    10
                } else {
                    space
                }
            };
            *score += *player as u32;
            if *score >= 1000 {
                println!("Player {} wins", i);
                break 'game;
            }
        }
    }
    println!("Player 2 score: {}", player_2_score);
    println!("{}", dice_rolls * player_2_score);

    let mut wins = [0, 0];
    let rolls: Vec<u32> = iproduct!(1..=3, 1..=3, 1..=3).map(|(a, b, c)| a + b + c).collect();
    println!("{:?}", rolls);
    let mut games: HashMap<[Player; 2], u64> = HashMap::from_iter([([Player { pos: p1, score: 0}, Player {pos: p2, score: 0}], 1)]);
    for i in (0..2).cycle() {
        let mut next_state = HashMap::new();
        for (&game, universes) in games.iter() {
            for &roll in rolls.iter() {
                let mut next_game = game;
                next_game[i].play(roll);
                if next_game[i].score >= 21 {
                    wins[i] += universes;
                } else {
                    *next_state.entry(next_game).or_default() += universes;
                }
            }
            
        }
        games = next_state;
        if games.is_empty() {
            break
        }
    }
        println!("{}", games.len());
    println!("{}", wins.iter().max().unwrap());
    Ok(())
}
