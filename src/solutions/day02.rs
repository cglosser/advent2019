use crate::solutions::Solution;
use crate::utilities::intcode;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        let tape: Vec<i32> = input
            .split(',')
            .map(|t| t.parse::<i32>().unwrap())
            .collect();
        let mut vm = intcode::VirtualMachine::new(tape, None);
        vm.set_word(1, 12).unwrap();
        vm.set_word(2, 02).unwrap();
        vm.run().unwrap();

        vm.get_word(0).unwrap().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        const TARGET: i32 = 19690720;

        let tape: Vec<i32> = input
            .split(',')
            .map(|t| t.parse::<i32>().unwrap())
            .collect();

        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut vm = intcode::VirtualMachine::new(tape.clone(), None);
                vm.set_word(1, noun).unwrap();
                vm.set_word(2, verb).unwrap();
                vm.run().unwrap();

                let output = vm.get_word(0).unwrap();
                if output == TARGET {
                    return (100 * noun + verb).to_string();
                }
            }
        }

        panic!("Could not match target")
    }
}
