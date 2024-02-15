use thiserror::Error;

use crate::sticker::{CornerSticker, EdgeSticker};

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid move: {0}")]
    InvalidMoveString(String),
    #[error("Invalid corner: {0}")]
    InvalidCornerString(String),
    #[error("Invalid corner: {0}")]
    InvalidEdgeString(String),
    #[error("Invalid corner cycle: {:?} - {:?} - {:?}", .0, .1, .2)]
    InvalidCornerCycle(CornerSticker, CornerSticker, CornerSticker),
    #[error("Invalid edge cycle: {:?} - {:?} - {:?}", .0, .1, .2)]
    InvalidEdgeCycle(EdgeSticker, EdgeSticker, EdgeSticker),
}
