mod solutions;
use solutions::*;

mod utilities;

fn main() {
    day01::part1();
    day01::part2();

    day02::part1();
    day02::part2();

    let day04 = day04::Day04 {};
    let day04_input = include_str!("../inputs/day04.txt");
    println!("Day 04a: {}", day04.part_one(day04_input));
    println!("Day 04b: {}", day04.part_two(day04_input));
}
