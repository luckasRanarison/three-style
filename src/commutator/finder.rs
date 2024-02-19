use super::types::{Commutator, Cycle};
use crate::{moves::MoveKind, sticker::Corner};

pub fn find_corner_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    todo!()
}

pub fn find_edge_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    todo!()
}
