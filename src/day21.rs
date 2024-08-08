use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

use std::collections::HashMap;

pub struct Solution {
    lines: Vec<String>,
    cache: HashMap<(usize, usize, usize, usize), (usize, usize)>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day21.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
            cache: HashMap::new(),
        }
    }

    fn part1(&mut self) -> i32 {
        let mut rolls = 0;
        let mut dice = (1..101).into_iter().cycle();
        let mut players = vec![3, 4];
        let mut scores = vec![0, 0];
        let mut current_player = 0;

        while scores[0] < 1000 && scores[1] < 1000 {
            let val = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
            players[current_player] = (players[current_player] + val) % 10;
            scores[current_player] += if players[current_player] == 0 {
                10
            } else {
                players[current_player]
            };

            current_player = (current_player + 1) % 2;
            rolls += 3;
        }

        let loosing_score = scores.iter().min().unwrap();
        *loosing_score * rolls
    }

    // Want it to return the number of games won by player 1
    fn game(&mut self, p1: usize, p2: usize, s1: usize, s2: usize) -> (usize, usize) {
        if s2 >= 21 {
            return (0, 1);
        }

        if let Some(winner) = self.cache.get(&(p1, p2, s1, s2)) {
            return *winner;
        }

        let mut won_by_players = (0, 0);
        for die1 in 1..=3 {
            for die2 in 1..=3 {
                for die3 in 1..=3 {
                    let dice_roll = die1 + die2 + die3;

                    // Player 1 is the one playing now
                    let mut new_p1 = (p1 + dice_roll) % 10;
                    if new_p1 == 0 {
                        new_p1 = 10;
                    };

                    // Update score
                    let new_s1 = s1 + new_p1;

                    let (w1, w2) = self.game(p2, new_p1, s2, new_s1);
                    won_by_players.0 += w2;
                    won_by_players.1 += w1;
                }
            }
        }

        // Write to cache
        self.cache.insert((p1, p2, s1, s2), won_by_players);
        won_by_players
    }

    fn part2(&mut self) -> usize {
        let (a, b) = self.game(3, 4, 0, 0);
        a.max(b)
    }

    pub fn solve(&mut self) {
        println!("========= DAY 21 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        let start = std::time::Instant::now();
        let part2 = self.part2();
        let part2_time = start.elapsed();
        println!("{:?} (took {:?})", part2, part2_time);
        println!();
    }
}
