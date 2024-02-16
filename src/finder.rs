use crate::{
    commutator::{Commutator, Cycle, ThreeCycle},
    moves::{find_parallel_moves, Move, MoveKind},
    state::Cube,
    sticker::{Corner, Edge},
};

pub fn find_corner_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Option<Vec<Commutator>> {
    let initial_state = Cube::default().corner_cycle(cycle.alt()).ok()?;
    let parallel_moves = find_parallel_moves(&allowed_moves)?;

    let finder = CornerCommutatorFinder {
        initial_state,
        cycle,
        parallel_moves,
        max_depth,
        moves: allowed_moves.to_vec(),
        results: Vec::new(),
    };

    finder.search()
}

struct CornerCommutatorFinder {
    initial_state: Cube,
    cycle: Cycle<Corner>,
    moves: Vec<MoveKind>,
    parallel_moves: (MoveKind, MoveKind),
    max_depth: u8,
    results: Vec<Commutator>,
}

impl CornerCommutatorFinder {
    fn search(mut self) -> Option<Vec<Commutator>> {
        todo!();

        if !self.results.is_empty() {
            Some(self.results)
        } else {
            None
        }
    }
}

pub fn find_edge_commutator(
    cycle: Cycle<Edge>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Option<Vec<Commutator>> {
    todo!()
}
