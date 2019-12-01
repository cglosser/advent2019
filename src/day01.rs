use std::fs::File;

use crate::utils;

const INPUT: &str = "inputs/day01_input.txt";

fn fuel(weight: &i64) -> i64 {
    std::cmp::max(0, weight / 3 - 2)
}

pub fn part1() {
    let numbers = utils::read(File::open(INPUT).unwrap()).unwrap();
    let result: i64 = numbers.iter().map(fuel).sum();
    println!("Day 01a: {}", result);
}

pub fn part2() {
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

    let numbers = utils::read(File::open(INPUT).unwrap()).unwrap();
    let result: i64 = numbers.iter().map(total_fuel).sum();
    println!("Day 01b: {}", result);
}
