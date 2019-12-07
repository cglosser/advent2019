mod instruction;
mod mode;
mod operation;

use instruction::*;
use mode::*;
use operation::*;

pub fn run(mut tape: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    loop {
        match Operation::try_from(tape[i]) {
            Ok(Operation::Add) => {
                let left = tape[i + 1] as usize;
                let right = tape[i + 2] as usize;
                let dest = tape[i + 3] as usize;
                tape[dest] = tape[left] + tape[right];
            }
            Ok(Operation::Multiply) => {
                let left = tape[i + 1] as usize;
                let right = tape[i + 2] as usize;
                let dest = tape[i + 3] as usize;
                tape[dest] = tape[left] * tape[right];
            }
            Ok(Operation::Halt) => break,
            _ => panic!("Unsupported operation"),
        }
        i += 4;
    }

    tape
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_examples() {
        assert_eq!(run(vec![1, 0, 0, 0, 99])[0], 2);
        assert_eq!(run(vec![2, 3, 0, 3, 99])[0], 2);
        assert_eq!(run(vec![2, 4, 4, 5, 99, 0])[0], 2);
        assert_eq!(run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])[0], 30);
    }
}
