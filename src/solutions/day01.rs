use crate::solutions::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> String {
        eval(input, |i| FuelSequence::new(i).nth(1).unwrap_or(0))
    }

    fn part_two(&self, input: &str) -> String {
        eval(input, |i| FuelSequence::new(i).skip(1).sum::<i32>())
    }
}

struct FuelSequence {
    curr: i32,
    next: i32,
}

impl FuelSequence {
    fn new(x: i32) -> FuelSequence {
        FuelSequence { curr: 0, next: x }
    }
}

impl Iterator for FuelSequence {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.curr = self.next;
        self.next = std::cmp::max(0, &self.curr / 3 - 2);

        match self.curr {
            0 => None,
            _ => Some(self.curr),
        }
    }
}

fn eval(input: &str, f: fn(i32) -> i32) -> String {
    input
        .split_ascii_whitespace()
        .map(|t| f(t.parse::<i32>().unwrap()))
        .sum::<i32>()
        .to_string()
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
        assert_eq!(2, FuelSequence::new(14).skip(1).sum::<i32>());
        assert_eq!(966, FuelSequence::new(1969).skip(1).sum::<i32>());
        assert_eq!(50346, FuelSequence::new(100756).skip(1).sum::<i32>());
    }
}
