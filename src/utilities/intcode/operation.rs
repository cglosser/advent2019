pub use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
    ReadInput,
    WriteOutput,
    Halt,
}

impl TryFrom<i32> for Operation {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Operation, Self::Error> {
        match value {
            1 => Ok(Operation::Add),
            2 => Ok(Operation::Multiply),
            3 => Ok(Operation::ReadInput),
            4 => Ok(Operation::WriteOutput),
            99 => Ok(Operation::Halt),
            _ => Err("Invalid conversion from i32 to Operation"),
        }
    }
}
