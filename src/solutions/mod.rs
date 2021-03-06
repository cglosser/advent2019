pub mod day01;
pub mod day02;
// pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;

    fn run(&self, input: &str) -> (String, String) {
        (self.part_one(input), self.part_two(input))
    }
}
