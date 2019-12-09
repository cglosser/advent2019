use crate::utilities::intcode::execution_error::ExecutionError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    pub fn try_from_opcode(opcode: i32, idx: u32) -> Result<ParameterMode, ExecutionError> {
        let place_value = 10i32.pow(idx + 2);
        let mode_value = ((opcode / place_value) % 10) as u8;

        match mode_value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            unknown => Err(ExecutionError::UnknownMode(unknown)),
        }
    }
}
