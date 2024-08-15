use std::collections::{BinaryHeap, HashMap, HashSet};
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

    fn part1(&mut self) -> Cost {
        let initial_map = Map::parse(&self.lines);
        Self::astar(initial_map)
    }

    fn part2(&mut self) -> Cost{
        let mut updated_lines = self.lines.clone();
        updated_lines.insert(3, "  #D#C#B#A#".to_string());
        updated_lines.insert(4, "  #D#B#A#C#".to_string());
        let initial_map = Map::parse(&updated_lines);
        Self::astar(initial_map)
    }

    fn astar(initial_map: Map) -> Cost {
        // Implement A* algorithm
        let mut open_set = BinaryHeap::new();
        let mut g_score = HashMap::new();

        open_set.push(Node {
            map: initial_map.clone(),
            g_score: 0,
            f_score: initial_map.heuristic(),
        });
        g_score.insert(initial_map.clone(), 0);

        while let Some(Node {
            map: current,
            g_score: current_g_score,
            f_score: _,
        }) = open_set.pop() {
            if current.is_final() {
                return current_g_score;
            }

            for (neighbor, d) in current.get_possible_next_states() {
                let neighbor_g_score = *g_score.get(&neighbor).unwrap_or(&Cost::MAX);
                let tentative_g_score = current_g_score + d;
                if tentative_g_score < neighbor_g_score {
                    g_score.insert(neighbor.clone(), tentative_g_score);

                    // Condition about the node being in open_set ? Not needed ?
                    open_set.push(Node {
                        f_score: tentative_g_score + neighbor.heuristic(),
                        map: neighbor,
                        g_score: tentative_g_score,
                    });
                }
            }
        }

        -1
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

type Cost = i32;

#[derive(Clone, Debug)]
struct Map {
    entities: Vec<Entity>,
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        // Sort the entites based on position, otherwise the comparison will fail
        let mut self_entities = self.entities.clone();
        let mut other_entities = other.entities.clone();

        self_entities.sort_by_key(|e| (e.x, e.y));
        other_entities.sort_by_key(|e| (e.x, e.y));

        self_entities == other_entities
    }
}

impl Eq for Map {}

impl Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Sort the entites based on position, otherwise the hashes will be different
        let mut entities = self.entities.clone();
        entities.sort_by_key(|e| (e.x, e.y));

        entities.hash(state);
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Entity {
    x: usize,
    y: usize,

    target_x: usize,
    max_y: usize,

    move_cost: Cost,
    pub entity_type: char,
}

impl Entity {
    pub fn parse(x: usize, y: usize, entity_type: char, max_y: usize) -> Option<Self> {
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
            max_y,
            target_x,
            move_cost,
            entity_type,
        })
    }

    pub fn in_theorical_final_spot(&self) -> bool {
        self.x == self.target_x && self.y >= 2
    }

    pub fn heuristic(&self) -> Cost {
        if self.in_theorical_final_spot() {
            return 0;
        }

        manathan_distance((self.x, self.y), (self.target_x, 2)) * self.move_cost
    }

    pub fn possible_next_steps(&self, entities: &HashMap<(usize, usize), Entity>) -> Vec<(usize, usize)>{
        let mut res = vec![];
        let hallway_entities: Vec<_> = (2..=self.max_y).map(|y| entities.get(&(self.target_x, y))).collect();

        // If it's in the hallway, it can only move to it's final column
        if self.y == 1 {
            // We need to find the deepest available spot
            let mut last_empty_spot = None;
            for (i, e) in hallway_entities.iter().enumerate() {
                if e.is_none() {
                    last_empty_spot = Some(i);
                } else {
                    break;
                }
            }

            if let Some(i) = last_empty_spot {
                // Check that the all the next one are full with the right type
                let mut can_move = true;
                for entity in &hallway_entities[i+1..] {
                    match entity {
                        Some(e) if e.entity_type == self.entity_type => continue,
                        None => continue,
                        _ => {
                            can_move = false;
                            break;
                        }
                    }
                }

                if can_move {
                    res.push((self.target_x, i + 2));
                }
            }
        }

        // If it's in one of the collumns, it can move to the hallway or it's in the final spot
        else {
            // This entity is in its final spot and all the spots after are taken
            let hallway_index = self.y - 2;
            if self.in_theorical_final_spot() && hallway_entities[hallway_index+1..].iter().all(|e| e.map(|f| f.entity_type == self.entity_type).unwrap_or(false)) {
                return res;
            }

            // It can move to the hallway
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

    pub fn cost_to_move(&self, (x, y): (usize, usize)) -> Cost {
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
        res
    }
}

pub fn manathan_distance(point_a: (usize, usize), point_b: (usize, usize)) -> Cost {
    let (x1, y1) = point_a;
    let (x2, y2) = point_b;

    (x1 as Cost - x2 as Cost).abs() + (y1 as Cost - y2 as Cost).abs()
}

impl Map {
    pub fn parse(lines: &Vec<String>) -> Self {
        let max_y = lines.len() - 2;
        let entities = lines.iter().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                Entity::parse(x, y, c, max_y)
            })
        })
            .flatten()
            .filter_map(|e| e)
            .collect::<Vec<_>>();
        
        Self {
            entities,
        }
    }

    pub fn heuristic(&self) -> Cost {
        self.entities.iter().map(|e| e.heuristic()).sum()
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

    pub fn get_possible_next_states(&self) -> Vec<(Self, Cost)> {
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
                };

                res.push((new_map, move_cost));
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    map: Map,
    g_score: Cost,
    f_score: Cost,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Inverted order because we want the smallest f_score, and BinaryHeap is a max heap
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}