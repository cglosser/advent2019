use crate::solutions::Solution;
use crate::utilities::intcode;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> String {
        let mut tape: Vec<i32> = input
            .split(',')
            .map(|t| t.parse::<i32>().unwrap())
            .collect();
        tape[1] = 12;
        tape[2] = 02;
        intcode::run(tape)[0].to_string()
    }

    fn part_two(&self, input: &str) -> String {
        const TARGET: i32 = 19690720;

        let tape: Vec<i32> = input
            .split(',')
            .map(|t| t.parse::<i32>().unwrap())
            .collect();

        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut program = tape.clone();
                program[1] = noun;
                program[2] = verb;
                let output = intcode::run(program)[0];
                if output == TARGET {
                    return (100 * noun + verb).to_string();
                }
            }
        }

        panic!("Could not match target")
    }
}
