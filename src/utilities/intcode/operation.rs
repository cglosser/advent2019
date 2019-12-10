use crate::utilities::intcode::{execution_error::ExecutionError, parameter_mode::ParameterMode};
use std::convert::TryFrom;

pub enum Operation {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    Halt,
}

impl TryFrom<i32> for Operation {
    type Error = ExecutionError;

    fn try_from(opcode: i32) -> Result<Operation, Self::Error> {
        match opcode % 100 {
            1 => Ok(Operation::Add(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
                ParameterMode::try_from_opcode(opcode, 2)?,
            )),
            2 => Ok(Operation::Multiply(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
                ParameterMode::try_from_opcode(opcode, 2)?,
            )),
            3 => Ok(Operation::Input(ParameterMode::try_from_opcode(opcode, 0)?)),
            4 => Ok(Operation::Output(ParameterMode::try_from_opcode(
                opcode, 0,
            )?)),
            5 => Ok(Operation::JumpIfTrue(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
            )),
            6 => Ok(Operation::JumpIfFalse(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
            )),
            7 => Ok(Operation::LessThan(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
                ParameterMode::try_from_opcode(opcode, 2)?,
            )),
            8 => Ok(Operation::Equals(
                ParameterMode::try_from_opcode(opcode, 0)?,
                ParameterMode::try_from_opcode(opcode, 1)?,
                ParameterMode::try_from_opcode(opcode, 2)?,
            )),
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
            if let Operation::Multiply(p1, p2, p3) = Operation::try_from(1002)? {
                assert_eq!(p1, ParameterMode::Position);
                assert_eq!(p2, ParameterMode::Immediate);
                assert_eq!(p3, ParameterMode::Position);
            } else {
                panic!();
            }

            if let Operation::Add(p1, p2, p3) = Operation::try_from(1101)? {
                assert_eq!(p1, ParameterMode::Immediate);
                assert_eq!(p2, ParameterMode::Immediate);
                assert_eq!(p3, ParameterMode::Position);
            } else {
                panic!();
            }

            Ok(())
        }
    }
}
