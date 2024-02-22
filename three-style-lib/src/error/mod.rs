use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Error)]
pub enum Error {
    #[error("Invalid move '{0}'")]
    InvalidMove(String),
    #[error("Invalid edge '{0}'")]
    InvalidEdgeString(String),
    #[error("Invalid corner '{0}'")]
    InvalidCornerString(String),
    #[error("Invalid cycle '{0}'")]
    InvalidThreeCycle(String),
}
