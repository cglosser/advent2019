#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    InvalidAddress,
    UnknownOpcode(i32),
    UnknownMode(u8),
}
