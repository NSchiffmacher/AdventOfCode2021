use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

use ndarray::prelude::*;
use ndarray_linalg::Norm;

pub struct Solution {
    content: String,
}

impl Solution {
    pub fn init() -> Self {
        Self {
            content: read_to_string("inputs/day19.txt").unwrap(),
        }
    }

    fn part1(&mut self) {
        // Parse
        let mut scanners = Vec::new();
        for block in self.content.split("\n\n") {
            let lines = block.lines().map(|s| s.to_string()).collect_vec();
            scanners.push(Scanner::from(lines));
        }

        // Solve
        println!("{:?}", scanners[1].compute_position_rotation(&scanners[0]));

        // for i in 0..scanners.len() {
        //     for j in i+1..scanners.len() {
        //         if let Some(()) = scanners[i].compute_position_rotation(&scanners[j]) {
        //             // println!("{} and {} match", i, j);
        //         }
        //     }
        // }
    }

    fn part2(&mut self) {}

    pub fn solve(&mut self) {
        println!("========= DAY 19 ========");
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

#[derive(Debug)]
pub struct Scanner {
    id: usize,
    scans: Vec<Array1<f64>>,
    position: Option<Array1<f64>>,
    self_distances: HashMap<u128, (usize, usize)>,
}

impl Scanner {
    pub fn from(lines: Vec<String>) -> Self {
        let id: usize = lines[0]
            .strip_prefix("--- scanner ")
            .unwrap()
            .strip_suffix(" ---")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let mut scans = Vec::new();
        for line in &lines[1..] {
            let (x, y, z) = line
                .split(",")
                .map(|s| s.parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            scans.push(arr1(&[x, y, z]));
        }

        // Compute distances on self
        let mut self_distances = HashMap::new();
        for i in 0..scans.len() {
            for j in i + 1..scans.len() {
                let dist = (&scans[i] - &scans[j]).norm();
                self_distances.insert((dist * 1_000_000_000.) as u128, (i, j));
            }
        }

        Self {
            id,
            scans,
            self_distances,
            position: None,
        }
    }

    pub fn compute_position_rotation(&self, _other: &Scanner) -> Option<()> {
        // // find matching distances
        // let mut matching_distances = Vec::new();
        // for (dist, (i, j)) in &self.self_distances {
        //     if let Some((k, l)) = other.self_distances.get(dist) {
        //         matching_distances.push(((*i, *j), (*k, *l)));
        //     }
        // }

        // if matching_distances.len() != 66 {
        //     return None;
        // }

        // let mut matching_indices = HashMap::new();
        // matching_indices.insert(matching_distances[0].0.0, matching_distances[0].1.0);
        // matching_indices.insert(matching_distances[0].0.1, matching_distances[0].1.1);
        // for _ in 0..10 {
        //     for ((i, j), (k, l)) in &matching_distances[1..] {
        //         if matching_indices.contains_key(i) && matching_indices.get(i) == Some(k) {
        //             matching_indices.insert(*j, *l);
        //         } else if matching_indices.contains_key(i) && matching_indices.get(i) == Some(l) {
        //             matching_indices.insert(*j, *k);
        //         } else if matching_indices.contains_key(j) && matching_indices.get(j) == Some(l) {
        //             matching_indices.insert(*i, *k);
        //         } else if matching_indices.contains_key(j) && matching_indices.get(j) == Some(k) {
        //             matching_indices.insert(*i, *l);
        //         }
        //     }
        // }

        // println!("{:?}", matching_indices);
        // println!("{:?}", matching_indices.len());

        // // let mat1 = Array2::from_shape_vec((3, 3), self.scans.iter().map(|v| v.to_vec()).flatten().collect_vec()).unwrap();
        // // println!("{:?}", mat1);

        Some(())
    }
}
