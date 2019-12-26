mod solutions;
use solutions::*;

#[macro_use]
extern crate lazy_static;

mod utilities;

fn main() {
    let day01 = day01::Day01 {};
    println!(
        "Day 01: {:?}",
        day01.run(include_str!("../inputs/day01.txt"))
    );

    let day02 = day02::Day02 {};
    println!(
        "Day 02: {:?}",
        day02.run(include_str!("../inputs/day02.txt"))
    );

    let day04 = day04::Day04 {};
    println!(
        "Day 04: {:?}",
        day04.run(include_str!("../inputs/day04.txt"))
    );

    let day05 = day05::Day05 {};
    println!(
        "Day 05: {:?}",
        day05.run(include_str!("../inputs/day05.txt"))
    );
    let day12 = day12::Day12 {};
    println!(
        "Day 12: {:?}",
        day12.run(include_str!("../inputs/day12.txt"))
    );
}
