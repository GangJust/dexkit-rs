use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("bridge input contains interior NUL byte: {0}")]
    InteriorNul(#[from] std::ffi::NulError),
    #[error("Dexkit bridge creation error: {0}")]
    BridgeCreate(&'static str),
    #[error("Dexkit bridge operation error: {0}")]
    BridgeOperation(&'static str),
    #[error("MUTF-8 decode error: {0}")]
    MUtf8Decode(&'static str),
    #[error("opcode error: {0}")]
    Opcode(String),
    #[error("unicode decode error: {0}")]
    UnicodeDecode(String),
}
