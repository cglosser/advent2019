use crate::utilities::intcode::{execution_error::ExecutionError, parameter_mode::ParameterMode};
use std::convert::TryFrom;

pub enum Operation {
    Add(ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    Halt,
}

impl TryFrom<i32> for Operation {
    type Error = ExecutionError;

    fn try_from(opcode: i32) -> Result<Operation, Self::Error> {
        match opcode % 100 {
            1 => Ok(Operation::Add(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
            )),
            2 => Ok(Operation::Multiply(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
            )),
            3 => Ok(Operation::Input),
            4 => Ok(Operation::Output(ParameterMode::try_from_opcode(
                opcode, 0,
            )?)),
            99 => Ok(Operation::Halt),
            _ => Err(ExecutionError::UnknownOpcode(opcode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod aoc_examples {
        use super::*;

        #[test]
        fn day05() -> Result<(), ExecutionError> {
            if let Operation::Multiply(p1, p2) = Operation::try_from(1002)? {
                assert_eq!(p1, ParameterMode::Position);
                assert_eq!(p2, ParameterMode::Immediate);
            } else {
                panic!();
            }

            if let Operation::Add(p1, p2) = Operation::try_from(1101)? {
                assert_eq!(p1, ParameterMode::Immediate);
                assert_eq!(p2, ParameterMode::Immediate);
            } else {
                panic!();
            }

            Ok(())
        }
    }
}
