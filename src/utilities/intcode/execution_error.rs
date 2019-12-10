#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    ImmediateModeWrite,
    InvalidAddress,
    MissingInput,
    UnknownMode(u8),
    UnknownOpcode(i32),
}
