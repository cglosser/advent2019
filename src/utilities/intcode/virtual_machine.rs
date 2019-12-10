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
            input: input.unwrap_or(Vec::new()),
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

        let opcode = self.get_cell(self.cursor)?;
        match Operation::try_from(opcode)? {
            Add(lmode, rmode) => {
                let left = self.get_argument(lmode, 1)?;
                let right = self.get_argument(rmode, 2)?;
                let dest = self.get_argument(ParameterMode::Immediate, 3)?;
                self.set_cell(Self::as_index(dest)?, left + right)?;
                self.cursor += 4;
            }
            Multiply(lmode, rmode) => {
                let left = self.get_argument(lmode, 1)?;
                let right = self.get_argument(rmode, 2)?;
                let dest = self.get_argument(ParameterMode::Immediate, 3)?;
                self.set_cell(Self::as_index(dest)?, left * right)?;
                self.cursor += 4;
            }
            Input => {
                let dest = self.get_argument(ParameterMode::Position, 1)?;
                let val = self.input.pop().ok_or(ExecutionError::InputError)?;
                self.set_cell(Self::as_index(dest)?, val)?;
                self.cursor += 2;
            }
            Output(mode) => {
                let val = self.get_argument(mode, 1)?;
                self.output.push(val);
                self.cursor += 2;
            }
            Halt => {
                self.halted = true;
            }
        }
        Ok(())
    }

    fn get_argument(&self, mode: ParameterMode, offset: usize) -> Result<i32> {
        match mode {
            ParameterMode::Immediate => self.access(self.cursor + offset),
            ParameterMode::Position => self.access_by_pointer(self.cursor + offset),
        }
    }

    fn access(&self, idx: usize) -> Result<i32> {
        self.tape
            .get(idx)
            .map(|i| *i)
            .ok_or(ExecutionError::InvalidAddress)
    }

    fn as_index<T: std::convert::TryInto<usize>>(idx: T) -> Result<usize> {
        idx.try_into().map_err(|_| ExecutionError::InvalidAddress)
    }

    fn access_by_pointer(&self, idx: usize) -> Result<i32> {
        let pointer = self.access(idx)?;
        self.access(pointer as usize)
    }

    pub fn set_cell(&mut self, idx: usize, value: i32) -> Result<()> {
        self.tape
            .get_mut(idx)
            .map(|i| *i = value)
            .ok_or(ExecutionError::InvalidAddress)
    }

    pub fn get_cell(&self, idx: usize) -> Result<i32> {
        self.tape
            .get(idx)
            .map(|i| *i)
            .ok_or(ExecutionError::InvalidAddress)
    }

    pub fn memory(&self) -> &[i32] {
        &self.tape
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
