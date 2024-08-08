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
        for line in read_to_string("inputs/day18.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self { lines }
    }

    fn part1(&mut self) -> T {
        let mut numbers = self.lines.iter().map(|line| parse(line));
        let first = numbers.next().unwrap();
        let addition_result = numbers.fold(first, |acc, num| add(acc, num));
        magnitude(&addition_result)
    }

    fn part2(&mut self) -> T {
        let pairs = self.lines.iter().map(|line| parse(line)).combinations(2);
        let mut max_mag = 0;
        for pair in pairs {
            let mag_addition_forward = magnitude(&add(pair[0].clone(), pair[1].clone()));
            let mag_addition_backward = magnitude(&add(pair[1].clone(), pair[0].clone()));
            max_mag = max_mag.max(mag_addition_forward).max(mag_addition_backward);
        }

        max_mag
    }

    pub fn solve(&mut self) {
        println!("========= DAY 18 ========");
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
type Number = Vec<Symbol>;
type Depth = usize;

#[derive(Debug, PartialEq, Clone)]
enum Symbol {
    Number(T),
    LBracket,
    RBracket,
    Comma,
}

fn parse(input: &str) -> Number {
    let mut symbols = Vec::new();

    for token in input.chars() {
        match token {
            '0'..='9' => symbols.push(Symbol::Number(token.to_digit(10).unwrap() as T)),
            '[' => symbols.push(Symbol::LBracket),
            ']' => symbols.push(Symbol::RBracket),
            ',' => symbols.push(Symbol::Comma),
            _ => (),
        }
    }

    symbols
}

fn format(input: &Number) -> String {
    let mut output = String::new();

    for symbol in input {
        match symbol {
            Symbol::Number(n) => output.push_str(&n.to_string()),
            Symbol::LBracket => output.push('['),
            Symbol::RBracket => output.push(']'),
            Symbol::Comma => output.push(','),
        }
    }

    output
}

fn find_first_regular_left(value: &Number, left_index: usize) -> Option<usize> {
    for i in (0..left_index).rev() {
        match value[i] {
            Symbol::Number(_) => return Some(i),
            _ => (),
        }
    }

    None
}

fn find_first_regular_right(value: &Number, right_index: usize) -> Option<usize> {
    for i in right_index + 1..value.len() {
        match value[i] {
            Symbol::Number(_) => return Some(i),
            _ => (),
        }
    }

    None
}

fn get_value(value: &Number, index: usize) -> T {
    match value[index] {
        Symbol::Number(n) => n,
        _ => panic!("Expected a number"),
    }
}

fn explode_at(number: &mut Number, left_index: usize) {
    if left_index == 0 {
        panic!("Cannot explode at the beginning of the list");
    }

    let right_index = left_index + 2; // Assume that left_index is a number and the next symbol is a comma, and the next one a number

    match find_first_regular_left(&number, left_index) {
        Some(index) => {
            number[index] =
                Symbol::Number(get_value(number, index) + get_value(number, left_index));
        }
        None => (),
    }

    match find_first_regular_right(&number, right_index) {
        Some(index) => {
            number[index] =
                Symbol::Number(get_value(number, index) + get_value(number, right_index));
        }
        None => (),
    }

    // Replace the pair from indices (inclusive) left_index - 1 (= LComma) and right_index + 1 (= RComma) with a number zero
    number.splice(left_index - 1..=right_index + 1, vec![Symbol::Number(0)]);
}

fn explode_once(value: &mut Number) -> bool {
    // Find the left most pair, if any, and if it's depth is greater than 4, explode it
    let mut depth = 0;
    for i in 0..value.len()-2 {
        match value[i] {
            Symbol::LBracket => depth += 1,
            Symbol::RBracket => depth -= 1,
            _ => (),
        }
        match (&value[i], &value[i + 1], &value[i + 2]) {
            (Symbol::Number(_), Symbol::Comma, Symbol::Number(_)) if depth > 4 => {
                explode_at(value, i);
                return true;
            }
            _ => (),
        }
    }

    false
}

fn split_at(number: &mut Number, regular_number_index: usize) {
    let value = get_value(number, regular_number_index);
    // let left_value = value.div_floor(&2);
    // let right_value = value.div_ceil(&2);
    let left_value = value / 2;
    let right_value = value - left_value;
    number.splice(
        regular_number_index..regular_number_index + 1,
        vec![
            Symbol::LBracket,
            Symbol::Number(left_value),
            Symbol::Comma,
            Symbol::Number(right_value),
            Symbol::RBracket,
        ],
    );
}

fn split_once(number: &mut Number) -> bool {
    for i in 0..number.len() {
        match number[i] {
            Symbol::Number(n) if n >= 10 => {
                split_at(number, i);
                return true;
            }
            _ => (),
        }
    }

    false
}

fn add(num_a: Number, num_b: Number) -> Number {
    let mut result = vec![Symbol::LBracket]
        .into_iter()
        .chain(num_a.into_iter())
        .chain(vec![Symbol::Comma])
        .chain(num_b.into_iter())
        .chain(vec![Symbol::RBracket])
        .collect_vec();

    // Apply reduction
    loop {
        if explode_once(&mut result) {
            continue;
        }

        let split = split_once(&mut result);
        if !split {
            break;
        }
    }
    result
}

fn magnitude(number: &Number) -> T {
    let mut expressions_queue = vec![];
    for token in number {
        match token {
            Symbol::LBracket | Symbol::Comma => (),
            Symbol::Number(n) => expressions_queue.push(*n),
            Symbol::RBracket => {
                if expressions_queue.len() < 2 {
                    panic!("Invalid expression, not enough operands");
                }
                let right = expressions_queue.pop().unwrap();
                let left = expressions_queue.pop().unwrap();
                expressions_queue.push(3 * left + 2 * right);
            }
        }
    }

    if expressions_queue.len() != 1 {
        panic!("Invalid expression, expressions_queue contains {} elements", expressions_queue.len());
    }

    expressions_queue.pop().unwrap()    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse1() {
        let input = "2";
        let expected = vec![Symbol::Number(2)];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn parse2() {
        let input = "[1,2]";
        let expected = vec![
            Symbol::LBracket,
            Symbol::Number(1),
            Symbol::Comma,
            Symbol::Number(2),
            Symbol::RBracket,
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn parse3() {
        let input = "[1,[2,3]]";
        let expected = vec![
            Symbol::LBracket,
            Symbol::Number(1),
            Symbol::Comma,
            Symbol::LBracket,
            Symbol::Number(2),
            Symbol::Comma,
            Symbol::Number(3),
            Symbol::RBracket,
            Symbol::RBracket,
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_format() {
        let input = vec![
            Symbol::LBracket,
            Symbol::Number(1),
            Symbol::Comma,
            Symbol::LBracket,
            Symbol::Number(2),
            Symbol::Comma,
            Symbol::Number(3),
            Symbol::RBracket,
            Symbol::RBracket,
        ];
        let expected = "[1,[2,3]]";
        assert_eq!(format(&input), expected);
    }

    #[test]
    fn test_explode_at() {
        let mut input = parse("[[[[[9,8],1],2],3],4]");
        explode_at(&mut input, 5);
        let expected = "[[[[0,9],2],3],4]";
        assert_eq!(format(&input), expected);
    }

    #[test]
    fn test_explode_once() {
        let mut input = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let expected = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        explode_once(&mut input);
        assert_eq!(format(&input), expected);
    }

    #[test]
    fn test_explode_once2() {
        let mut input = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        explode_once(&mut input);
        assert_eq!(format(&input), expected);
    }

    #[test]
    fn test_split_at() {
        let mut input = vec![Symbol::Number(10)];
        let expected = "[5,5]";
        split_at(&mut input, 0);
        assert_eq!(format(&input), expected);
    }

    #[test]
    fn test_split_at2() {
        let mut input = vec![Symbol::Number(11)];
        let expected = "[5,6]";
        split_at(&mut input, 0);
        assert_eq!(format(&input), expected);
    }

    #[test]
    fn test_add1() {
        let a = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = parse("[1,1]");
        let addition = add(a, b);
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(format(&addition), expected);
    }
}
