#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    InvalidAddress,
    InputError,
    UnknownOpcode(i32),
    UnknownMode(u8),
}
