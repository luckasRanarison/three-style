use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid move: {0}")]
    InvalidMoveString(String),
    #[error("Invalid corner: {0}")]
    InvalidCornerString(String),
    #[error("Invalid corner: {0}")]
    InvalidEdgeString(String),
}
