mod day01;
mod day02;
mod day04;
mod problem;
mod utils;

use problem::Problem;

fn main() {
    day01::part1();
    day01::part2();

    day02::part1();
    day02::part2();

    let day04 = day04::Day04 {};
    println!("Day 04a: {}", day04.part_one("146810-612564"));
    println!("Day 04b: {}", day04.part_two("146810-612564"));
}
