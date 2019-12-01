use std::fs::File;

use crate::utils;

const INPUT: &str = "inputs/day01_input.txt";

struct FuelSequence {
    curr: i64,
    next: i64,
}

impl FuelSequence {
    fn new(x: i64) -> FuelSequence {
        FuelSequence { curr: 0, next: x }
    }
}

impl Iterator for FuelSequence {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.curr = self.next;
        self.next = std::cmp::max(0, &self.curr / 3 - 2);

        match self.curr {
            0 => None,
            _ => Some(self.curr),
        }
    }
}

fn eval(identifier: &str, f: fn(&i64) -> i64) {
    let numbers = utils::read(File::open(INPUT).unwrap()).unwrap();
    let result: i64 = numbers.iter().map(f).sum();
    println!("{}: {}", identifier, result);
}

pub fn part1() {
    eval("Day 01a", |x| FuelSequence::new(*x).nth(1).unwrap_or(0));
}

pub fn part2() {
    eval("Day 01b", |x| FuelSequence::new(*x).skip(1).sum::<i64>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fuel_examples() {
        assert_eq!(None, FuelSequence::new(0).nth(1));
        assert_eq!(Some(2), FuelSequence::new(12).nth(1));
        assert_eq!(Some(2), FuelSequence::new(14).nth(1));
        assert_eq!(Some(654), FuelSequence::new(1969).nth(1));
        assert_eq!(Some(33_583), FuelSequence::new(100_756).nth(1));
    }

    #[test]
    fn total_fuel_examples() {
        assert_eq!(2, FuelSequence::new(14).skip(1).sum::<i64>());
        assert_eq!(966, FuelSequence::new(1969).skip(1).sum::<i64>());
        assert_eq!(50346, FuelSequence::new(100756).skip(1).sum::<i64>());
    }
}
