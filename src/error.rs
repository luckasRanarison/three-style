use thiserror::Error;

use crate::sticker::{Corner, Edge};

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid move: {0}")]
    InvalidMoveString(String),
    #[error("Invalid corner: {0}")]
    InvalidCornerString(String),
    #[error("Invalid corner: {0}")]
    InvalidEdgeString(String),
    #[error("Invalid corner cycle: {:?} - {:?} - {:?}", .0, .1, .2)]
    InvalidCornerCycle(Corner, Corner, Corner),
    #[error("Invalid edge cycle: {:?} - {:?} - {:?}", .0, .1, .2)]
    InvalidEdgeCycle(Edge, Edge, Edge),
}
