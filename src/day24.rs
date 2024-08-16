use std::collections::HashSet;
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
        for line in read_to_string("inputs/day24.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self { lines }
    }

    fn part1(&mut self) -> Value {
        let digits = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut invalid_states = HashSet::new();
        generate_model_number(&digits, 0, 0, 0, &mut invalid_states).unwrap()
    }

    fn part2(&mut self) -> Value {
        let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut invalid_states = HashSet::new();
        generate_model_number(&digits, 0, 0, 0, &mut invalid_states).unwrap()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 24 ========");
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

type Value = i128;

fn generate_model_number(digits_to_try: &[Value; 9], mut model_number: Value, original_z: Value, depth: usize, invalid_states: &mut HashSet<(Value, usize)>) -> Option<Value> {
    if invalid_states.contains(&(original_z, depth)) || depth == 14 {
        return None;
    }

    let div_values = [1, 1, 1, 26, 26, 1, 26, 26, 1, 26, 1, 1, 26, 26];
    let add_values = [12, 14, 11, -9, -7, 11, -1, -16, 11, -15, 10, 12, -4, 0];
    let second_add_values = [15, 12, 15, 12, 15, 2, 11, 15, 10, 2, 0, 0, 15, 15];

    model_number *= 10;

    for i in 0..9 {
        let w = digits_to_try[i];
        let mut x = original_z;
        let mut y = 25;
        let mut z = original_z;

        x %= 26;
        z /= div_values[depth];
        x += add_values[depth];
        x = if x == w { 1 } else { 0 };
        x = if x == 0 { 1 } else { 0 };
        y *= x;
        y += 1;
        z *= y;
        y = w;
        y += second_add_values[depth];
        y *= x;
        z += y;

        // We are at the end of the number and we have a solution, return it
        if z == 0 && depth == 13 {
            return Some(model_number + w);
        }

        // Check the next depth
        if let Some(result) = generate_model_number(digits_to_try, model_number + w, z, depth + 1, invalid_states) {
            return Some(result);
        }
    }

    // Won't work with this z value at this depth, no matter what the other numbers are
    invalid_states.insert((original_z, depth));
    None
}
