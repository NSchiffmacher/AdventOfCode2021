use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day23.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self { lines }
    }

    fn part1(&mut self) -> T {
        let initial_map = Map::parse(&self.lines);

        // Implement A* algorithm
        let mut open_set = BTreeSet::new();
        let mut g_score = HashMap::new();
        let mut came_from: HashMap<Map, Map> = HashMap::new();
        // let mut f_score = HashMap::new();

        open_set.insert(initial_map.clone());
        g_score.insert(initial_map.clone(), 0);
        // f_score.insert(initial_map.clone(), initial_map.heuristic());

        while let Some(current) = open_set.pop_first() {
            if current.is_final() {
                let cost = current.current_cost;

                // Reconstruct path
                let mut path = vec![current.clone()];
                let mut current = current.clone();

                while let Some(previous) = came_from.get(&current) {
                    path.push(previous.clone());
                    current = previous.clone();
                }

                // Print path
                for map in path.iter().rev() {
                    println!("{}", map);
                }
                
                return cost;
            }

            for neighbor in current.get_possible_next_states() {
                let current_g_score = *g_score.get(&neighbor).unwrap_or(&T::MAX);
                let tentative_g_score = neighbor.current_cost;
                
                if tentative_g_score < current_g_score {
                    g_score.insert(neighbor.clone(), tentative_g_score);
                    came_from.insert(neighbor.clone(), current.clone());
                    // f_score.insert(neighbor.clone(), tentative_g_score + neighbor.heuristic());

                    // if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                    // }
                }

            }

        }

        -1
    }

    fn part2(&mut self) {
        // let initial_map = Map::parse(&self.lines);

        // println!("\n\nInitial map:");
        // println!("{}", initial_map);

        // let entities = initial_map.build_entities();
        // let entity = entities.get(&(11, 1)).unwrap();

        // for neighbor in initial_map.get_possible_next_states() {
        //     println!("Neighbor: ");
        //     println!("{}", neighbor);
        // }
    }

    pub fn solve(&mut self) {
        println!("========= DAY 23 ========");
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

type T = i32;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Map {
    entities: Vec<Entity>,
    current_cost: T,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Entity {
    x: usize,
    y: usize,
    target_x: usize,

    move_cost: T,
    pub entity_type: char,
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let f_score_self = self.current_cost + self.heuristic();
        let f_score_other = other.current_cost + other.heuristic();

        f_score_self.partial_cmp(&f_score_other)
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let f_score_self = self.current_cost + self.heuristic();
        let f_score_other = other.current_cost + other.heuristic();

        f_score_self.cmp(&f_score_other)
    }
}

impl Entity {
    pub fn parse(x: usize, y: usize, entity_type: char) -> Option<Self> {
        let (target_x, move_cost) = match entity_type {
            'A' => (3, 1),
            'B' => (5, 10),
            'C' => (7, 100),
            'D' => (9, 1000),
            _ => return None,
        };

        Some(Self {
            x,
            y,
            target_x,
            move_cost,
            entity_type,
        })
    }

    pub fn in_theorical_final_spot(&self) -> bool {
        self.x == self.target_x && self.y >= 2
    }

    pub fn possible_next_steps(&self, entities: &HashMap<(usize, usize), Entity>) -> Vec<(usize, usize)>{
        let mut res = vec![];

        // This entity is in its final spot, it's not moving anymore
        if self.in_theorical_final_spot() && (self.y == 3 || (self.y == 2 && entities.get(&(self.target_x, 3)).map_or(false, |e| e.in_theorical_final_spot()))) {
            return res;
        }

        // If it's in the hallway, it can only move to it's final column
        if self.y == 1 {
            // Can it go to (target_x, 3)
            let deep_final_spot_entity = entities.get(&(self.target_x, 3));
            let shallow_final_spot_entity = entities.get(&(self.target_x, 2));

            // Deep final spot is not empty
            if let Some(e) = deep_final_spot_entity {
                if e.in_theorical_final_spot() && shallow_final_spot_entity.is_none() {
                    res.push((self.target_x, 2));
                }
            } 
            
            // Deep final spot is empty
            else {
                res.push((self.target_x, 3));
            } 

        }

        // If it's in one of the collumns (and not in it's final spot), it can move to the hallway
        else {
            for x in 1..=11 {
                if entities.get(&(x, 1)).is_none() && x != 3 && x != 5 && x != 7 && x != 9 {
                    res.push((x, 1));
                }
            }
        }

        // We need to filter the paths that aren't blocked
        res.retain(|(x, y)| self.has_available_path_towards(*x, *y, entities));
        res
    }

    pub fn cost_to_move(&self, (x, y): (usize, usize)) -> T {
        let dist = manathan_distance((self.x, self.y), (x, y));

        dist * self.move_cost
    }

    fn has_available_path_towards(&self, target_x: usize, target_y: usize, entities: &HashMap<(usize, usize), Entity>) -> bool {
        let blocked_cells = entities.keys().cloned().collect::<HashSet<_>>();
        let path = self.build_path(target_x, target_y);

        // A path is available if there's no blocked cells in it, ie. the intersection is empty
        blocked_cells.intersection(&path).count() == 0
    }

    fn build_path(&self, target_x: usize, target_y: usize) -> HashSet<(usize, usize)> {
        let mut res = HashSet::new();

        // Target y and self.y will always be different at the start
        if target_y == self.y {
            panic!("Unexpected in build_path. Initial {:?}, target {:?}", (self.x, self.y), (target_x, target_y));
        }

        // Figure out the x column where we move vertically 
        let x = if self.y == 1 {
            self.target_x
        } else {
            self.x
        };

        // We do not care about building it in the right order, we only care about the cells in the middle
        // Also, this path does not include the start
        // Include y cells
        for y in self.y.min(target_y)..=self.y.max(target_y) {
            res.insert((x, y));
        }

        // Include x cells (can only be in the hallway)
        for x in self.x.min(target_x)..=self.x.max(target_x) {
            res.insert((x, 1));
        }

        // Remove the start
        res.remove(&(self.x, self.y));

        // if self.x == 9 && self.y == 3 && target_x == 11 && target_y == 1 {
        //     println!("Found ! {:?}", res.iter().sorted().collect::<Vec<_>>());
        // }

        res
    }
}

pub fn manathan_distance(point_a: (usize, usize), point_b: (usize, usize)) -> T {
    let (x1, y1) = point_a;
    let (x2, y2) = point_b;

    (x1 as T - x2 as T).abs() + (y1 as T - y2 as T).abs()
}

impl Map {
    pub fn parse(lines: &Vec<String>) -> Self {
        let entities = lines.iter().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                Entity::parse(x, y, c)
            })
        })
            .flatten()
            .filter_map(|e| e)
            .collect::<Vec<_>>();
        

        Self {
            entities,
            current_cost: 0,
        }
    }

    pub fn heuristic(&self) -> T {
        0 // TODO 
    }

    pub fn is_final(&self) -> bool {
        self.entities.iter().all(|e| e.in_theorical_final_spot())
    }

    fn build_entities(&self) -> HashMap<(usize, usize), Entity> {
        let mut entities = HashMap::new();
        for entity in &self.entities {
            entities.insert((entity.x, entity.y), entity.clone());
        }

        if entities.len() != self.entities.len() {
            panic!("Entities have repeated positions");
        }

        entities
    }

    pub fn get_possible_next_states(&self) -> Vec<Self> {
        let mut res = vec![];
        let entities = self.build_entities();
        
        for (pos, entity) in &entities {
            for next_step in entity.possible_next_steps(&entities) {
                let mut new_entities = entities.clone();
                let new_entity = Entity {
                    x: next_step.0,
                    y: next_step.1,
                    ..entity.clone()
                };

                // Move the entity
                new_entities.remove(pos).unwrap();
                new_entities.insert(next_step, new_entity.clone());
                let move_cost = entity.cost_to_move(next_step);

                // Build the new map
                let new_map = Self {
                    entities: new_entities.values().cloned().collect(),
                    current_cost: self.current_cost + move_cost,
                };

                res.push(new_map);
            }
        }

        res
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec![vec!['#'; 13]; 5];
        
        // Write empty symbols
        for x in 1..12 {
            chars[1][x] = '.';
            if x >= 3 && x <= 9 && x % 2 == 1 {
                chars[2][x] = '.';
                chars[3][x] = '.';
            }
        }

        // Write spaces
        for x in vec![0, 1, 11, 12] {
            for y in 3..=4 {
                chars[y][x] = ' ';
            }
        }

        // Write the entities
        let entities = self.build_entities();
        for ((x, y), e) in entities {
            chars[y][x] = e.entity_type;
        }

        writeln!(f, "")?;
        for line in chars {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }

        Ok(())
    }
}