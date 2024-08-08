use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day22.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self { lines }
    }

    fn part1(&mut self) -> T{
        let cuboids = self
            .lines
            .iter()
            .map(|line| Cuboid::from_line(line))
            .filter(|cuboid| cuboid.valid_for_part_1())
            .collect_vec();
        let mut cores: Vec<Cuboid> = vec![];

        for cuboid in cuboids {
            let mut to_add = if cuboid.added { vec![cuboid.clone()] } else { vec![] };
            for core_cuboid in &cores {
                let intersection = cuboid.intersection(core_cuboid);
                if let Some(intersection) = intersection {
                    to_add.push(intersection);
                }
            }
            cores.extend(to_add);
        }

        cores.iter().fold(0, |acc, cuboid| acc + cuboid.signed_volume())
    }

    fn part2(&mut self) -> T {
        let cuboids = self
            .lines
            .iter()
            .map(|line| Cuboid::from_line(line))
            .collect_vec();
        let mut cores: Vec<Cuboid> = vec![];

        for cuboid in cuboids {
            let mut to_add = if cuboid.added { vec![cuboid.clone()] } else { vec![] };
            for core_cuboid in &cores {
                let intersection = cuboid.intersection(core_cuboid);
                if let Some(intersection) = intersection {
                    to_add.push(intersection);
                }
            }
            cores.extend(to_add);
        }

        cores.iter().fold(0, |acc, cuboid| acc + cuboid.signed_volume())
    }

    pub fn solve(&mut self) {
        println!("========= DAY 22 ========");
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

type T = i64;

#[derive(Debug, Clone)]
struct Cuboid {
    added: bool,
    min_x: T,
    max_x: T,
    min_y: T,
    max_y: T,
    min_z: T,
    max_z: T,
}

impl Cuboid {
    pub fn new(min_x: T, max_x: T, min_y: T, max_y: T, min_z: T, max_z: T, added: bool) -> Self {
        Self {
            added,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min_x = self.min_x.max(other.min_x);
        let max_x = self.max_x.min(other.max_x);
        let min_y = self.min_y.max(other.min_y);
        let max_y = self.max_y.min(other.max_y);
        let min_z = self.min_z.max(other.min_z);
        let max_z = self.max_z.min(other.max_z);

        if min_x <= max_x && min_y <= max_y && min_z <= max_z { // TODO: Check if need equality ?
            Some(Self::new(min_x, max_x, min_y, max_y, min_z, max_z, !other.added))
        } else {
            None
        }
    }

    pub fn valid_for_part_1(&self) -> bool {
        self.min_x >= -50 && self.max_x <= 50 && self.min_y >= -50 && self.max_y <= 50 && self.min_z >= -50 && self.max_z <= 50
    }

    pub fn from_line(line: &str) -> Self {
        let (state_str, remainder) = line.split_once(" ").unwrap();
        let added = match state_str {
            "on" => true,
            "off" => false,
            _ => panic!("Invalid state"),
        };

        let mut values = Vec::new();
        for part in remainder.split(",") {
            let (_axis, range) = part.split_once("=").unwrap();
            let (min, max) = range.split_once("..").unwrap();
            values.push(min.parse().unwrap());
            values.push(max.parse().unwrap());
        }

        let (min_x, max_x, min_y, max_y, min_z, max_z) = values.into_iter().collect_tuple().unwrap();
        Self::new(min_x, max_x, min_y, max_y, min_z, max_z, added)
    }

    pub fn signed_volume(&self) -> T {
        let sign = if self.added {
            1
        } else {
            -1
        };
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1) * (self.max_z - self.min_z + 1) * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "on x=-7..46,y=-33..20,z=-18..35";
        let cuboid = Cuboid::from_line(line);
        assert_eq!(cuboid.min_x, -7);
        assert_eq!(cuboid.max_x, 46);
        assert_eq!(cuboid.min_y, -33);
        assert_eq!(cuboid.max_y, 20);
        assert_eq!(cuboid.min_z, -18);
        assert_eq!(cuboid.max_z, 35);
        assert_eq!(cuboid.added, true);
    }
}