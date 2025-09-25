use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Dexkit Bridge Creation Error: {0}")]
    BridgeCreateError(String),
    #[error("Dexkit Bridge Operation Error: {0}")]
    BridgeOperationError(String),
    #[error("MUtf8 Decode Error: {0}")]
    MUtf8DecodeError(String),
    #[error("Opcode Error: {0}")]
    OpcodeError(String),
}
