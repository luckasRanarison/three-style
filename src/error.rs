use crate::{
    commutator::Cycle,
    sticker::{Corner, Edge},
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid move: {0}")]
    InvalidMoveString(String),
    #[error("Invalid corner: {0}")]
    InvalidCornerString(String),
    #[error("Invalid corner: {0}")]
    InvalidEdgeString(String),
    #[error("Invalid corner cycle: {:?} - {:?} - {:?}", .0.first, .0.second, .0.third)]
    InvalidCornerCycle(Cycle<Corner>),
    #[error("Invalid edge cycle: {:?} - {:?} - {:?}", .0.first, .0.second, .0.third)]
    InvalidEdgeCycle(Cycle<Edge>),
}
