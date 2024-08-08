use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    algo: Vec<usize>,
    image: HashSet<(isize, isize)>,
    fill: isize,
    current_min_x: isize,
    current_max_x: isize,
    current_min_y: isize,
    current_max_y: isize,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day20.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let algo = lines[0]
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid c in algo"),
            })
            .collect_vec();

        let mut image = HashSet::new();
        for (y, line) in lines[2..].iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    image.insert((x as isize, y as isize));
                }
            }
        }

        Self {
            algo,
            image,
            current_min_x: 0,
            current_max_x: lines[2].len() as isize - 1,
            current_min_y: 0,
            current_max_y: lines.len() as isize - 1 - 2,
            lines,
            fill: 0,
        }
    }

    fn out_of_image(&self, x: isize, y: isize) -> bool {
        x < self.current_min_x
            || x > self.current_max_x
            || y < self.current_min_y
            || y > self.current_max_y
    }

    fn enhance_image(&mut self) {
        let mut image = HashSet::new();
        for x in self.current_min_x - 1..=self.current_max_x + 1 {
            for y in self.current_min_y - 1..=self.current_max_y + 1 {
                let mut base = 2usize.pow(8);
                let mut value = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if self.image.contains(&(x + dx, y + dy))
                            || (self.fill == 1 && self.out_of_image(x + dx, y + dy))
                        {
                            value += base;
                            // print!("1");
                        } else {
                            // print!("0")
                        }
                        base /= 2;
                    }
                }
                // println!(" = {value} -> {}", self.algo[value]);

                if self.algo[value] == 1 {
                    image.insert((x, y));
                }
            }
        }

        self.current_max_x += 1;
        self.current_max_y += 1;
        self.current_min_x -= 1;
        self.current_min_y -= 1;
        if self.algo[0] == 1 {
            self.fill = (self.fill + 1) % 2;
        }
        self.image = image;
    }

    fn print_image(&self) {
        for y in self.current_min_y..=self.current_max_y {
            for x in self.current_min_x..=self.current_max_x {
                if self.image.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn part1(&mut self) -> usize {
        // 5765
        self.enhance_image();
        self.enhance_image();

        self.image.len()
    }

    fn part2(&mut self) -> usize {
        // 18509
        for _ in 0..48 {
            self.enhance_image();
        }

        self.image.len()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 20 ========");
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
