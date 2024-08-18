// Inspired by https://www.ericburden.work/blog/2021/12/29/advent-of-code-2021-day-19/

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    content: String,
}

impl Solution {
    pub fn init() -> Self {
        let content = read_to_string("inputs/day25.txt").unwrap();

        Self { content }
    }

    fn part1(&mut self) -> usize{
        let mut map = Map::try_from(self.content.as_str()).unwrap();
        let mut seen = HashSet::new();
        let mut step = 0;

        loop {
            let map_string = map.to_string();
            if seen.contains(map_string.as_str()) {
                break;
            }

            seen.insert(map_string);
            map.tick();
            step += 1;
        }

        step
    }

    pub fn solve(&mut self) {
        println!("========= DAY 25 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        println!("MERRY CHRISTMAS");
        println!();
    }
}


type Coordinate = i32;

#[derive(Debug, PartialEq, Clone)]
enum Creature {
    EastFacing,
    SouthFacing,
}

impl Creature {
    fn get_delta(&self) -> (Coordinate, Coordinate) {
        match self {
            Creature::EastFacing => (1, 0),
            Creature::SouthFacing => (0, 1),
        }
    }
}

struct Map {
    width: Coordinate,
    height: Coordinate,

    creatures: HashMap<(Coordinate, Coordinate), Creature>,
}

impl TryFrom<&str> for Map {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value.lines().collect::<Vec<_>>();
        let width = lines[0].len() as Coordinate;
        let height = lines.len() as Coordinate;

        let mut creatures = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = (x as Coordinate, y as Coordinate);
                match c {
                    '>' => { creatures.insert(coord, Creature::EastFacing); }
                    'v' => { creatures.insert(coord, Creature::SouthFacing); }
                    '.' => (), // empty space
                    _ => return Err(format!("Invalid character: {}", c).into()),
                };
            }
        }

        Ok(Self { width, height, creatures })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(creature) = self.creatures.get(&(x, y)) {
                    match creature {
                        Creature::EastFacing => write!(f, ">")?,
                        Creature::SouthFacing => write!(f, "v")?,
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    pub fn tick(&mut self) {
        let east_moves = self.can_move(Creature::EastFacing);
        self.apply_moves(east_moves);

        let south_moves = self.can_move(Creature::SouthFacing);
        self.apply_moves(south_moves);
    }

    pub fn can_move(&mut self, creature_type: Creature) -> HashMap<(Coordinate, Coordinate), (Coordinate, Coordinate)> {
        let mut result = HashMap::new();

        for (coord, creature) in self.creatures.iter() {
            if creature == &creature_type {
                let (x, y) = coord;
                let (dx, dy) = creature.get_delta();
                let new_coord = self.wrap((x + dx, y + dy));

                if !self.creatures.contains_key(&new_coord) {
                    result.insert(*coord, new_coord);
                }
            }
        }

        result
    }

    fn apply_moves(&mut self, moves: HashMap<(Coordinate, Coordinate), (Coordinate, Coordinate)>) {
        for (old_coord, new_coord) in moves {
            let creature = self.creatures.remove(&old_coord).expect(format!("No creature at {:?}", old_coord).as_str());
            self.creatures.insert(new_coord, creature);
        }
    }

    fn wrap(&self, (x, y): (Coordinate, Coordinate)) -> (Coordinate, Coordinate) {
        (x % self.width, y % self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let input = "..........\n.>........\n..v....v>.\n..........\n";
        let map = Map::try_from(input).unwrap();
        assert_eq!(map.to_string(), input);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 4);
    }

    #[test]
    fn test_tick_1d() {
        let input = "...>>>>>...\n";
        let mut map = Map::try_from(input).unwrap();
        map.tick();
        assert_eq!(map.to_string(), "...>>>>.>..\n");
    }

    #[test]
    fn test_tick_2d() {
        let input = "..........\n.>v....v..\n.......>..\n..........\n";
        let expected = "..........\n.>........\n..v....v>.\n..........\n";
        let mut map = Map::try_from(input).unwrap();
        map.tick();
        assert_eq!(map.to_string(), expected);
    }

    #[test]
    fn test_multiple_ticks() {
        let input = "...>...\n.......\n......>\nv.....>\n......>\n.......\n..vvv..\n";
        let expected = ">......\n..v....\n..>.v..\n.>.v...\n...>...\n.......\nv......\n";
        let steps = 4;

        let mut map = Map::try_from(input).unwrap();
        (0..steps).for_each(|_| map.tick());
        assert_eq!(map.to_string(), expected);
    }
}