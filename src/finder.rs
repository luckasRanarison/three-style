use crate::{
    commutator::{Commutator, Cycle, ThreeCycle},
    moves::Move,
    state::Cube,
    sticker::{Corner, Edge},
};

pub fn find_corner_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[Move],
    max_depth: u8,
) -> Option<Commutator> {
    todo!()
}

pub fn find_edge_commutator(
    cycle: Cycle<Edge>,
    allowed_moves: &[Move],
    max_depth: u8,
) -> Option<Commutator> {
    todo!()
}
