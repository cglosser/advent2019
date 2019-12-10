use crate::solutions::Solution;
use crate::utilities::intcode;

pub struct Day05 {}

fn run(input: &str, i: i32) -> String {
    let args: Vec<i32> = vec![i];
    let mut vm = intcode::VirtualMachine::new::<Vec<_>>(
        input
            .split(',')
            .map(|t| t.parse::<i32>().unwrap())
            .collect(),
        Some(args),
    );
    vm.run().expect("Could not run TEST program");
    vm.output().last().unwrap().to_string()
}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> String {
        run(input, 1)
    }

    fn part_two(&self, input: &str) -> String {
        run(input, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_data_part_one() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut vm = intcode::VirtualMachine::new(program, None);
        assert!(vm.run().is_ok());
        assert!(vm.memory().iter().eq(vec![1002, 4, 3, 4, 99].iter()))
    }

    #[test]
    fn example_data_part_two() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // Check if input < 8
        let mut vm = intcode::VirtualMachine::new(program.clone(), Some(vec![7]));
        assert!(vm.run().is_ok());
        assert_eq!(*vm.output().last().unwrap(), 999);

        // Check if input = 8
        let mut vm = intcode::VirtualMachine::new(program.clone(), Some(vec![8]));
        assert!(vm.run().is_ok());
        assert_eq!(*vm.output().last().unwrap(), 1000);

        // Check if input > 8
        let mut vm = intcode::VirtualMachine::new(program.clone(), Some(vec![9]));
        assert!(vm.run().is_ok());
        assert_eq!(*vm.output().last().unwrap(), 1001);
    }
}
