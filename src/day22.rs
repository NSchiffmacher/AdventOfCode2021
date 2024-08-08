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

    fn part1(&mut self) {
        // Parse
        let rectangles = self
            .lines
            .iter()
            .map(|line| Rectangle::from_line(line))
            .collect_vec();

        // Solve
        // let mut world = vec![Rectangle::new(Interval::new(0, 0), Interval::new(0, 0), Interval::new(0, 0), false)];
        // for rectangle in rectangles {
        //     let mut new_world = Vec::new();
        //     for world_rectangle in world {
        //         // new_world.append(&mut world_rectangle.update(&rectangle));
        //     }
        //     world = new_world;
        // }

        let aabb1 = Rectangle::from_min_max([0, 0, 0], [5, 5, 5], false);

        let aabb2 = Rectangle::from_min_max([3, 3, 3], [8, 8, 8], false);

        if let Some(overlap) = Rectangle::calculate_overlap(&aabb1, &aabb2) {
            println!("Overlap AABB: {:?}", overlap);

            let external = Rectangle::generate_external_aabbs(&aabb1, &aabb2, &overlap);
            println!("External AABBs:");
            for e in external {
                println!("{:?}", e);
            }
        } else {
            println!("No overlap between the AABBs.");
        }
    }

    fn part2(&mut self) {}

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

#[derive(Debug, Clone)]
pub struct Interval {
    pub min: isize,
    pub max: isize,
}

impl Interval {
    pub fn new(min: isize, max: isize) -> Self {
        Self { min, max }
    }

    pub fn from_str(s: &str) -> Self {
        let (_, s) = s.split_once("=").unwrap();
        let (min, max) = s.split_once("..").unwrap();
        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,

    pub state: bool,
}

impl Rectangle {
    pub fn new(x: Interval, y: Interval, z: Interval, state: bool) -> Self {
        Self { x, y, z, state }
    }

    pub fn from_line(line: &str) -> Self {
        let (state, coordinates) = line.split(" ").collect_tuple().unwrap();
        let (x, y, z) = coordinates.split(",").collect_tuple().unwrap();

        Self {
            x: Interval::from_str(x),
            y: Interval::from_str(y),
            z: Interval::from_str(z),
            state: if state == "on" { true } else { false },
        }
    }

    fn calculate_overlap(aabb1: &Rectangle, aabb2: &Rectangle) -> Option<Rectangle> {
        let overlap_min: [isize; 3] = [
            aabb1.x.min.max(aabb2.x.min),
            aabb1.y.min.max(aabb2.y.min),
            aabb1.z.min.max(aabb2.z.min),
        ];

        let overlap_max: [isize; 3] = [
            aabb1.x.max.min(aabb2.x.max),
            aabb1.y.max.min(aabb2.y.max),
            aabb1.z.max.min(aabb2.z.max),
        ];

        if overlap_min[0] < overlap_max[0]
            && overlap_min[1] < overlap_max[1]
            && overlap_min[2] < overlap_max[2]
        {
            Some(Rectangle {
                x: Interval::new(overlap_min[0], overlap_max[0]),
                y: Interval::new(overlap_min[1], overlap_max[1]),
                z: Interval::new(overlap_min[2], overlap_max[2]),
                state: false,
            })
        } else {
            None
        }
    }

    fn from_min_max(min_point: [isize; 3], max_point: [isize; 3], state: bool) -> Self {
        Self {
            x: Interval::new(min_point[0], max_point[0]),
            y: Interval::new(min_point[1], max_point[1]),
            z: Interval::new(min_point[2], max_point[2]),
            state,
        }
    }

    fn generate_external_aabbs(
        aabb1: &Rectangle,
        aabb2: &Rectangle,
        overlap_aabb: &Rectangle,
    ) -> Vec<Rectangle> {
        let mut external_aabbs = Vec::new();

        // Add AABBs covering regions to the left of the overlap
        if aabb1.x.min < overlap_aabb.x.min {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb1.x.min, aabb1.y.min, aabb1.z.min],
                [overlap_aabb.x.min, aabb1.y.max, aabb1.z.max],
                false,
            ));
        }

        if aabb2.x.min < overlap_aabb.x.min {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb2.x.min, aabb2.y.min, aabb2.z.min],
                [overlap_aabb.x.min, aabb2.y.max, aabb2.z.max],
                false,
            ));
        }

        // Add AABBs covering regions to the right of the overlap
        if aabb1.x.max > overlap_aabb.x.max {
            external_aabbs.push(Rectangle::from_min_max(
                [overlap_aabb.x.max, aabb1.y.max, aabb1.y.min],
                [aabb1.x.max, aabb1.y.max, aabb1.z.max],
                false,
            ));
        }

        if aabb2.x.max > overlap_aabb.x.max {
            external_aabbs.push(Rectangle::from_min_max(
                [overlap_aabb.x.max, aabb2.y.max, aabb2.y.min],
                [aabb2.x.max, aabb2.y.max, aabb2.z.max],
                false,
            ));
        }

        /////////////////////////////////////////////////
        // Add AABBs covering regions to the left of the overlap
        if aabb1.y.min < overlap_aabb.y.min {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb1.x.min, aabb1.y.min, aabb1.z.min],
                [aabb1.x.min, overlap_aabb.y.max, aabb1.z.max],
                false,
            ));
        }

        if aabb2.y.min < overlap_aabb.y.min {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb2.x.min, aabb2.y.min, aabb2.z.min],
                [aabb2.x.min, overlap_aabb.y.max, aabb2.z.max],
                false,
            ));
        }

        // Add AABBs covering regions to the right of the overlap
        if aabb1.y.max > overlap_aabb.y.max {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb1.x.max, overlap_aabb.y.max, aabb1.y.min],
                [aabb1.x.max, aabb1.y.max, aabb1.z.max],
                false,
            ));
        }

        if aabb2.y.max > overlap_aabb.y.max {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb2.x.max, overlap_aabb.y.max, aabb2.y.min],
                [aabb2.x.max, aabb2.y.max, aabb2.z.max],
                false,
            ));
        }

        /////////////////////////////////////////////////
        // Add AABBs covering regions to the left of the overlap
        if aabb1.z.min < overlap_aabb.z.min {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb1.x.min, aabb1.y.min, aabb1.z.min],
                [aabb1.x.min, aabb1.y.max, overlap_aabb.z.max],
                false,
            ));
        }

        if aabb2.z.min < overlap_aabb.z.min {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb2.x.min, aabb2.y.min, aabb2.z.min],
                [aabb2.x.min, aabb2.y.max, overlap_aabb.z.max],
                false,
            ));
        }

        // Add AABBs covering regions to the right of the overlap
        if aabb1.z.max > overlap_aabb.z.max {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb1.x.max, aabb1.y.max, overlap_aabb.y.min],
                [aabb1.x.max, aabb1.y.max, aabb1.z.max],
                false,
            ));
        }

        if aabb2.y.max > overlap_aabb.y.max {
            external_aabbs.push(Rectangle::from_min_max(
                [aabb2.x.max, overlap_aabb.y.max, aabb2.y.min],
                [aabb2.x.max, aabb2.y.max, aabb2.z.max],
                false,
            ));
        }

        external_aabbs
    }
}
