use std::fs::File;

use crate::utils;

const INPUT: &str = "inputs/day01_input.txt";

pub fn fuel(weight: &i64) -> i64 {
    std::cmp::max(0, weight / 3 - 2)
}

fn total_fuel(weight: &i64) -> i64 {
    fn helper(total: i64, weight: i64) -> i64 {
        let f = fuel(&weight);
        match f {
            0 => total,
            _ => helper(total + f, f),
        }
    }

    helper(0, weight.clone())
}

pub fn part1() {
    let numbers = utils::read(File::open(INPUT).unwrap()).unwrap();
    let result: i64 = numbers.iter().map(fuel).sum();
    println!("Day 01a: {}", result);
}

pub fn part2() {
    let numbers = utils::read(File::open(INPUT).unwrap()).unwrap();
    let result: i64 = numbers.iter().map(total_fuel).sum();
    println!("Day 01b: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fuel_examples() {
        assert_eq!(0, fuel(&0));
        assert_eq!(2, fuel(&12));
        assert_eq!(2, fuel(&14));
        assert_eq!(654, fuel(&1969));
        assert_eq!(33583, fuel(&100756));
    }

    #[test]
    fn total_fuel_examples() {
        assert_eq!(2, total_fuel(&14));
        assert_eq!(966, total_fuel(&1969));
        assert_eq!(50346, total_fuel(&100756));
    }
}
