use crate::solutions::Solution;
use crate::utilities::intcode;

pub struct Day05 {}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        let args: Vec<i32> = vec![1];
        let mut vm = intcode::VirtualMachine::new::<Vec<_>>(
            input
                .split(',')
                .map(|t| t.parse::<i32>().unwrap())
                .collect(),
            Some(args),
        );
        vm.run().expect("Could not run TEST program");
        println!("{:?}", vm.output().iter().map(|i| *i).collect::<Vec<i32>>());
        "Potato".to_owned()
    }

    fn part_two(&self, input: &str) -> String {
        "Potato".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
