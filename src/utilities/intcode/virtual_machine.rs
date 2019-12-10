// Inspired by the intcode crate by Daniel Dulaney

use crate::utilities::intcode::{
    execution_error::ExecutionError, operation::Operation, parameter_mode::ParameterMode,
};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    tape: Vec<i32>,
    cursor: usize,
    halted: bool,
    input: Vec<i32>,
    output: Vec<i32>,
}

type Result<T> = std::result::Result<T, ExecutionError>;

impl VirtualMachine {
    pub fn new<T: Into<Vec<i32>>>(tape: T, input: Option<Vec<i32>>) -> VirtualMachine {
        VirtualMachine {
            tape: tape.into(),
            cursor: 0,
            halted: false,
            input: input.unwrap_or(Vec::new()).into(),
            output: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted {
            self.step()?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        use Operation::*;

        let opcode = self.get_word(self.cursor)?;
        match Operation::try_from(opcode)? {
            Add(mode1, mode2, mode3) => {
                let a = self.get_argument(mode1, 1)?;
                let b = self.get_argument(mode2, 2)?;
                self.set_argument(mode3, 3, a + b)?;
                self.cursor += 4;
            }
            Multiply(mode1, mode2, mode3) => {
                let a = self.get_argument(mode1, 1)?;
                let b = self.get_argument(mode2, 2)?;
                self.set_argument(mode3, 3, a * b)?;
                self.cursor += 4;
            }
            Input(mode) => {
                let val = self.input.pop().ok_or(ExecutionError::MissingInput)?;
                self.set_argument(mode, 1, val)?;
                self.cursor += 2;
            }
            Output(mode) => {
                let val = self.get_argument(mode, 1)?;
                self.output.push(val);
                self.cursor += 2;
            }
            JumpIfTrue(mode1, mode2) => {
                let a = self.get_argument(mode1, 1)?;
                if a != 0 {
                    self.cursor = Self::as_index(self.get_argument(mode2, 2)?)?;
                } else {
                    self.cursor += 3;
                }
            }
            JumpIfFalse(mode1, mode2) => {
                let a = self.get_argument(mode1, 1)?;
                if a == 0 {
                    self.cursor = Self::as_index(self.get_argument(mode2, 2)?)?;
                } else {
                    self.cursor += 3;
                }
            }
            LessThan(mode1, mode2, mode3) => {
                let a = self.get_argument(mode1, 1)?;
                let b = self.get_argument(mode2, 2)?;
                self.set_argument(mode3, 3, (a < b) as i32)?;
                self.cursor += 4;
            }
            Equals(mode1, mode2, mode3) => {
                let a = self.get_argument(mode1, 1)?;
                let b = self.get_argument(mode2, 2)?;
                self.set_argument(mode3, 3, (a == b) as i32)?;
                self.cursor += 4;
            }
            Halt => {
                self.halted = true;
            }
        }
        Ok(())
    }

    fn get_argument(&self, mode: ParameterMode, offset: usize) -> Result<i32> {
        match mode {
            ParameterMode::Immediate => self.get_word(self.cursor + offset),
            ParameterMode::Position => self.get_word_by_pointer(self.cursor + offset),
        }
    }

    fn set_argument(&mut self, mode: ParameterMode, offset: usize, value: i32) -> Result<()> {
        match mode {
            ParameterMode::Immediate => Err(ExecutionError::ImmediateModeWrite),
            ParameterMode::Position => self.set_word_by_pointer(self.cursor + offset, value),
        }
    }

    pub fn get_word(&self, idx: usize) -> Result<i32> {
        self.tape
            .get(idx)
            .map(|i| *i)
            .ok_or(ExecutionError::InvalidAddress)
    }

    fn get_word_by_pointer(&self, idx: usize) -> Result<i32> {
        let pointer = self.get_word(idx)?;
        self.get_word(Self::as_index(pointer)?)
    }

    pub fn set_word(&mut self, idx: usize, value: i32) -> Result<()> {
        self.tape
            .get_mut(idx)
            .map(|i| *i = value)
            .ok_or(ExecutionError::InvalidAddress)
    }

    fn set_word_by_pointer(&mut self, idx: usize, value: i32) -> Result<()> {
        self.set_word(Self::as_index(self.get_word(idx)?)?, value)
    }

    fn as_index<T: std::convert::TryInto<usize>>(idx: T) -> Result<usize> {
        idx.try_into().map_err(|_| ExecutionError::InvalidAddress)
    }

    #[allow(dead_code)]
    pub fn memory(&self) -> &[i32] {
        &self.tape
    }

    pub fn output(&self) -> &[i32] {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod aoc_examples {
        use super::*;

        fn check(input: Vec<i32>, output: Vec<i32>) {
            let mut vm = VirtualMachine::new(input, None);
            vm.run().expect("Could not run intcode");
            assert!(vm.memory().iter().eq(output.iter()));
        }

        #[test]
        fn day02() {
            check(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
            check(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
            check(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
            check(
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            );
        }

        #[test]
        fn day05() {
            let mut vm = VirtualMachine::new(vec![1101, 100, -1, 4, 0], None);
            vm.run().expect("Could not run intcode");
            check(vec![1101, 100, -1, 4, 0], vec![1101, 100, -1, 4, 99]);
        }
    }
}
