pub use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i32> for Mode {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err("Invalid conversion from i32 to Mode"),
        }
    }
}
