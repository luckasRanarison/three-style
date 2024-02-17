use std::collections::HashMap;

use crate::{
    commutator::{Commutator, Cycle, ThreeCycle},
    facelet::Facelet,
    moves::{find_parallel_moves, Move, MoveKind},
    state::Cube,
    sticker::{Corner, Edge},
};

pub fn find_corner_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    let finder = CornerCommutatorFinder::new(cycle, allowed_moves, max_depth);

    if let Some(finder) = finder {
        finder.search()
    } else {
        Vec::new()
    }
}

struct CornerCommutatorFinder {
    cycle: Cycle<Corner>,
    initial_state: Cube,
    positions: HashMap<Facelet, Facelet>,
    allowed_moves: Vec<MoveKind>,
    parallel_moves: (MoveKind, MoveKind),
    current_moves: Vec<Move>,
    max_depth: u8,
    results: Vec<Commutator>,
}

impl CornerCommutatorFinder {
    fn new(cycle: Cycle<Corner>, allowed_moves: &[MoveKind], max_depth: u8) -> Option<Self> {
        let parallel_moves = find_parallel_moves(&allowed_moves)?;
        let initial_state = Cube::default().corner_cycle(cycle.alt()).ok()?;
        let mut positions = HashMap::new();
        positions.insert(cycle.first_facelet(), initial_state[cycle.third_facelet()]);
        positions.insert(cycle.second_facelet(), initial_state[cycle.first_facelet()]);
        positions.insert(cycle.third_facelet(), initial_state[cycle.second_facelet()]);

        Some(Self {
            cycle,
            initial_state,
            positions,
            parallel_moves,
            max_depth,
            allowed_moves: allowed_moves.to_vec(),
            current_moves: Vec::new(),
            results: Vec::new(),
        })
    }

    fn search(mut self) -> Vec<Commutator> {
        let initial_state = self.initial_state.clone();
        let (move_a, move_b) = self.parallel_moves;
        let interchange_moves = move_a
            .get_move_variants()
            .into_iter()
            .chain(move_b.get_move_variants().into_iter())
            .collect::<Vec<_>>();
        self.find_corner_interchange(initial_state, &interchange_moves, 0);
        self.results
    }

    fn find_corner_interchange(&mut self, state: Cube, allowed_moves: &[Move], depth: u8) {
        if depth == self.max_depth {
            return;
        }

        for m in allowed_moves {}
    }
}

pub fn find_edge_commutator(
    cycle: Cycle<Edge>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    todo!()
}
