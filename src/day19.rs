use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{self, Write};
use std::sync::LazyLock; 

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    content: String,
    offsets: Vec<Vector>,
}

impl Solution {
    pub fn init() -> Self {
        Self {
            content: read_to_string("inputs/day19.txt").unwrap(),
            offsets: vec![],
        }
    }

    fn part1(&mut self) -> usize {
        // Parse
        let scanners = Scanners::try_from(self.content.clone()).unwrap();
        let (mapped_scanners, offsets) = mapscanners(&scanners);
        self.offsets = offsets;
        mapped_scanners.0
            .values()
            .fold(HashSet::new(), |acc, scanner| acc.union(scanner).copied().collect::<HashSet<_>>())
            .len()
    }

    fn part2(&mut self) -> Coordinate {
        // We need to find the max distance between two beacons
        itertools::iproduct!(self.offsets.iter(), self.offsets.iter())
            .map(|(a, b)| manhattan_distance(a, b))
            .max()
            .unwrap()
    }

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

type Coordinate = i32; // The type used to represent a coordinate value
type Vector = (Coordinate, Coordinate, Coordinate); // A vector is a tuple of (x, y, z)
type ScannerId = usize; // The type used to represent a scanner id
type Beacon = Vector; // A beacon is a represented by it's position
type Scanner = HashSet<Beacon>; // A scanner is a set of beacons
type Basis = (Vector, Vector, Vector); // A basis is a tuple of three coordinates

#[derive(Debug)]
struct Scanners(HashMap<ScannerId, Scanner>); // A collection of scanners is accessed by their id

const ROTATIONS: [[Coordinate; 3]; 24] = [
    [0,   0,   0], [90,   0,   0], [180,   0,   0], [270,   0,   0],
    [0,  90,   0], [90,  90,   0], [180,  90,   0], [270,  90,   0],
    [0, 180,   0], [90, 180,   0], [180, 180,   0], [270, 180,   0],
    [0, 270,   0], [90, 270,   0], [180, 270,   0], [270, 270,   0],
    [0,   0,  90], [90,   0,  90], [180,   0,  90], [270,   0,  90],
    [0,   0, 270], [90,   0, 270], [180,   0, 270], [270,   0, 270]
];

static BASES: LazyLock<Vec<Basis>> = LazyLock::new(|| {
    ROTATIONS.iter().map(|rotation| {
        (
            rotation_xyz((1, 0, 0), rotation[0], rotation[1], rotation[2]),
            rotation_xyz((0, 1, 0), rotation[0], rotation[1], rotation[2]),
            rotation_xyz((0, 0, 1), rotation[0], rotation[1], rotation[2]),
        )
    }).collect_vec()
});

impl TryFrom<String> for Scanners {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut scanners = HashMap::new();

        // Separate the different blocks
        for block in value.split("\n\n") {
            let lines = block.lines().collect_vec();

            // Read the scanner ID 
            let scanner_id = lines[0]
                .strip_prefix("--- scanner ").ok_or("Invalid scanner ID")?
                .strip_suffix(" ---").ok_or("Invalid scanner ID")?
                .parse()?;

            // Read the scanner beacons
            let mut scanner = Scanner::new();
            for line in &lines[1..] {
                let beacon = line
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .ok_or("Invalid beacon")?;
                scanner.insert(beacon);
            }
            scanners.insert(scanner_id, scanner);
        }

        Ok(Self(scanners))
    }
}

fn cos(angle: Coordinate) -> Coordinate {
    match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("Invalid angle"),
    }
}

fn sin(angle: Coordinate) -> Coordinate {
    match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("Invalid angle"),
    }
}

fn rotation_x((x, y, z): Vector, angle: Coordinate) -> Vector {
    (
        x,
        y * cos(angle) - z * sin(angle),
        y * sin(angle) + z * cos(angle),
    )
}

fn rotation_y((x, y, z): Vector, angle: Coordinate) -> Vector {
    (
        x * cos(angle) + z * sin(angle),
        y,
        -x * sin(angle) + z * cos(angle),
    )
}

fn rotation_z((x, y, z): Vector, angle: Coordinate) -> Vector {
    (
        x * cos(angle) - y * sin(angle),
        x * sin(angle) + y * cos(angle),
        z,
    )
}

fn rotation_xyz(mut vector: Vector, angle_x: Coordinate, angle_y: Coordinate, angle_z: Coordinate) -> Vector {
    vector = rotation_x(vector, angle_x);
    vector = rotation_y(vector, angle_y);
    rotation_z(vector, angle_z)
}

fn translate_beacon(beacon: Beacon, basis: Basis) -> Beacon {
    let (bx1, by1, bz1) = basis.0;
    let (bx2, by2, bz2) = basis.1;
    let (bx3, by3, bz3) = basis.2;
    let (x, y, z) = beacon;

    (
        bx1 * x + by1 * y + bz1 * z,
        bx2 * x + by2 * y + bz2 * z,
        bx3 * x + by3 * y + bz3 * z,
    )
}

fn translate_scanner(scanner: &Scanner, basis: Basis) -> Scanner {
    scanner.iter().map(|beacon| translate_beacon(*beacon, basis)).collect()
}

fn incommon(a: &Scanner, b: &Scanner) -> Option<(Scanner, Vector)> {
    for basis in BASES.iter() {
        let translated_b = translate_scanner(b, *basis);
        for (a_beacon, b_beacon) in itertools::iproduct!(a.iter(), translated_b.iter()) {
            let offset = (
                a_beacon.0 - b_beacon.0,
                a_beacon.1 - b_beacon.1,
                a_beacon.2 - b_beacon.2,
            );
            let shifted_b: Scanner = translated_b.iter().map(|beacon| {
                (
                    beacon.0 + offset.0,
                    beacon.1 + offset.1,
                    beacon.2 + offset.2,
                )
            }).collect();
            if a.intersection(&shifted_b).count() >= 12 {
                return Some((shifted_b, offset))
            }
        }
    }

    None
}

fn mapscanners(scanners: &Scanners) -> (Scanners, Vec<Vector>) {
    let mut mapped_scanners = HashMap::new();
    mapped_scanners.insert(0, scanners.0[&0].clone());
    let mut offsets = vec![(0, 0, 0)];
    let mut searched = HashSet::new();

    while scanners.0.keys().collect::<HashSet<_>>().difference(&mapped_scanners.keys().collect::<HashSet<_>>()).count() > 0 {
        for id1 in scanners.0.keys() {
            if searched.contains(id1) || !mapped_scanners.contains_key(id1) {
                continue;
            }

            for (id2, scanner2) in scanners.0.iter() {
                if mapped_scanners.contains_key(id2) || id1 == id2 {
                    continue;
                }

                if let Some((common, offset)) = incommon(mapped_scanners.get(id1).unwrap(), scanner2) {
                    mapped_scanners.insert(*id2, common);
                    offsets.push(offset);
                    // println!("Found common beacons between scanner {} and scanner {} => {} remaining", id1, id2, scanners.0.keys().collect::<HashSet<_>>().difference(&mapped_scanners.keys().collect::<HashSet<_>>()).count());
                }
            }

            searched.insert(id1);
        }
    }

    (Scanners(mapped_scanners), offsets)
}

fn manhattan_distance(a: &Vector, b: &Vector) -> Coordinate {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}