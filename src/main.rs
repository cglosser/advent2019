mod solutions;
use solutions::*;

#[macro_use]
extern crate itertools;

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

    let day06 = day06::Day06 {};
    println!(
        "Day 06: {:?}",
        day06.run(include_str!("../inputs/day06.txt"))
    );

    let day07 = day07::Day07 {};
    println!(
        "Day 07: {:?}",
        day07.run(include_str!("../inputs/day07.txt"))
    );

    let day08 = day08::Day08 {};
    println!(
        "Day 08: {:?}",
        day08.run(include_str!("../inputs/day08.txt"))
    );
}
