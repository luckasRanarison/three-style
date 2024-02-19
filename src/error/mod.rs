use crate::{
    commutator::types::Cycle,
    sticker::{Corner, Edge},
};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Error)]
pub enum Error {
    #[error("Invalid move '{0}'")]
    InvalidMove(String),
    #[error("Invalid edge '{0}'")]
    InvalidEdgeString(String),
    #[error("Invalid corner '{0}'")]
    InvalidCornerString(String),
    #[error("Invalid edge cycle '{} - {} - {}'", .0.first(), .0.second(), .0.third())]
    InvalidEdgeCycle(Cycle<Edge>),
    #[error("Invalid corder cycle '{} - {} - {}'", .0.first(), .0.second(), .0.third())]
    InvalidCornerCycle(Cycle<Corner>),
}
