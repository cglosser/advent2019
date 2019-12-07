pub use crate::utilities::intcode::{mode::Mode, operation::Operation};
pub use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub modes: [Mode; 3],
}

impl TryFrom<i32> for Instruction {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Instruction, Self::Error> {
        let mut digits: Vec<i32> = Vec::with_capacity(5);
        digits.extend(
            value
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .rev(),
        );
        digits.resize(5, 0);

        let mut modes = [Mode::Position, Mode::Position, Mode::Position];
        for (i, &digit) in digits.iter().skip(2).enumerate() {
            modes[i] = Mode::try_from(digit)?;
        }

        Ok(Instruction {
            operation: Operation::try_from(digits[0] + 10 * digits[1])?,
            modes: modes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_examples() {
        assert_eq!(
            Instruction::try_from(1002),
            Ok(Instruction {
                operation: Operation::Multiply,
                modes: [Mode::Position, Mode::Immediate, Mode::Position]
            })
        );

        assert_eq!(
            Instruction::try_from(1101),
            Ok(Instruction {
                operation: Operation::Add,
                modes: [Mode::Immediate, Mode::Immediate, Mode::Position]
            })
        );
    }
}
