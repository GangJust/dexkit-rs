use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Dexkit Bridge Creation Error: {0}")]
    BridgeCreateError(String),
}
