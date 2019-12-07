mod instruction;
mod mode;
mod operation;

use instruction::{Instruction, *};

type Tape = Vec<i32>;

pub fn run(mut tape: Tape) -> (Tape, Vec<i32>) {
    let value_from_tape = |mode: &Mode, arg: i32| -> i32 {
        match mode {
            Mode::Position => tape[tape[arg as usize] as usize],
            Mode::Immediate => tape[arg as usize],
        }
    };

    let mut output: Vec<i32> = Vec::new();

    let mut i = 0;
    loop {
        match Instruction::try_from(tape[i]) {
            Ok(Instruction {
                operation: op,
                modes: [m0, m1, m2],
            }) => match op {
                Operation::Add | Operation::Multiply => {
                    let left = value_from_tape(&m0, tape[i + 1]);
                    let right = value_from_tape(&m1, tape[i + 2]);
                    let dest = value_from_tape(&m2, tape[i + 3]);
                    tape[dest as usize] = if op == Operation::Add {
                        left + right
                    } else {
                        left * right
                    };
                }
                Operation::ReadInput => {
                    // Not implemented
                }
                Operation::WriteOutput => {
                    output.push(value_from_tape(&m0, tape[i + 1]));
                }
                Operation::Halt => break,
            },
            _ => panic!("Instruction failure"),
        }
        i += 4;
    }

    (tape, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_examples() {
        assert_eq!(run(vec![1, 0, 0, 0, 99]).0[0], 2);
        assert_eq!(run(vec![2, 3, 0, 3, 99]).0[0], 2);
        assert_eq!(run(vec![2, 4, 4, 5, 99, 0]).0[0], 2);
        assert_eq!(run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).0[0], 30);
    }
}
