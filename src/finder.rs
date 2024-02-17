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

#[derive(Debug)]
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
        let positions = cycle
            .to_facelets()
            .into_iter()
            .map(|f| (f, initial_state[&f]))
            .collect();

        Some(Self {
            cycle,
            positions,
            initial_state,
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
        self.find_interchange(initial_state, &interchange_moves, 0);
        self.results
    }

    fn find_interchange(&mut self, state: Cube, allowed_moves: &[Move], depth: u8) {
        if depth == self.max_depth {
            return;
        }

        for m in allowed_moves {
            let move_state = state.clone().apply_move(*m);

            if let Some(source) = self.check_interchange(move_state) {
                self.find_insertion(state.clone(), *m, allowed_moves, source, depth + 1);
            }
        }

        self.finde_setup_moves(state, allowed_moves, depth);
    }

    fn check_interchange(&self, move_state: Cube) -> Option<(Facelet, Facelet)> {
        for (slot, facelet) in &self.positions {
            let new_facelet = move_state[slot];

            if *facelet != new_facelet && self.positions.contains_key(&new_facelet) {
                return self
                    .positions
                    .iter()
                    .find(|(_, &f)| f != new_facelet && f != *facelet)
                    .map(|(s, f)| (s.clone(), f.clone()));
            }
        }

        None
    }

    fn find_insertion(
        &mut self,
        state: Cube,
        interchange: Move,
        interchange_moves: &[Move],
        source: (Facelet, Facelet),
        depth: u8,
    ) {
        if self.max_depth - depth < 3 {
            return;
        }

        let wrapper_moves = self
            .allowed_moves
            .iter()
            .filter(|&m| m.is_side() && *m != interchange.kind)
            .flat_map(|m| m.get_move_variants())
            .collect::<Vec<_>>();
        let second_moves = interchange.kind.inverse().get_move_variants();

        for wm in wrapper_moves {
            let frist_state = state.clone().apply_move(wm);

            for sm in &second_moves {
                let second_state = frist_state.clone().apply_move(*sm);
                let last_state = second_state.apply_move(wm.inverse());

                if self.check_insertion(last_state, source) {
                    let insertion = vec![wm, *sm, wm.inverse()];
                    self.add_commutator(interchange, insertion, state.clone());
                }
            }
        }

        self.finde_setup_moves(state, interchange_moves, depth);
    }

    fn check_insertion(&self, move_state: Cube, source: (Facelet, Facelet)) -> bool {
        let (source_slot, source_facelet) = source;

        for (slot, facelet) in &self.positions {
            if *slot != source_slot
                && move_state[slot] == source_facelet
                && self.positions.contains_key(facelet)
            {
                return true;
            }
        }

        false
    }

    fn finde_setup_moves(&mut self, state: Cube, interchange_moves: &[Move], depth: u8) {
        if depth == self.max_depth {
            return;
        }

        let setup_moves = self
            .allowed_moves
            .iter()
            .flat_map(|m| m.get_move_variants())
            .collect::<Vec<_>>();

        for m in setup_moves {
            if let Some(last) = self.current_moves.last() {
                if last.inverse() == m {
                    continue;
                }
            }

            self.current_moves.push(m);
            self.find_interchange(state.clone().apply_move(m), interchange_moves, depth + 1);
            self.current_moves.pop();
        }
    }

    // FIXME: Correctly guess the order
    fn add_commutator(&mut self, interchange: Move, insertion: Vec<Move>, state: Cube) {
        let setup = if !self.current_moves.is_empty() {
            Some(self.current_moves.clone())
        } else {
            None
        };
        let mut commutator = Commutator {
            setup,
            interchange,
            insertion,
            insertion_first: false,
        };

        if !commutator.solve(state.clone()) {
            commutator.insertion_first = true;
        }

        if commutator.solve(state.clone()) {
            self.results.push(commutator);
        }
    }
}

pub fn find_edge_commutator(
    cycle: Cycle<Edge>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    todo!()
}
