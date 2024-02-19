use super::types::{Commutator, Cycle};
use crate::{
    facelet::{moves::FaceletPermutation, Facelet, FaceletCube},
    moves::{Alg, Inverse, Move, MoveKind},
    sticker::Corner,
};

#[derive(Debug, PartialEq, Clone)]
struct Slot {
    original: Facelet,
    current: Facelet,
}

pub fn find_corner_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    let initial_state = match FaceletCube::try_from(cycle.inverse()) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let interchange_moves = match find_opposite_moves(&allowed_moves) {
        Some(value) => value
            .into_iter()
            .flat_map(|m| m.to_moves())
            .collect::<Vec<_>>(),
        None => return Vec::new(),
    };
    let slots = cycle
        .to_facelets()
        .into_iter()
        .map(|f| Slot {
            original: f,
            current: initial_state[f],
        })
        .collect();
    let allowed_moves = allowed_moves
        .into_iter()
        .flat_map(MoveKind::to_moves)
        .collect::<Vec<_>>();

    let finder = CornerFinder {
        slots,
        current_moves: Vec::new(),
        max_depth,
        results: Vec::new(),
    };

    finder.search(initial_state, &interchange_moves, &allowed_moves)
}

fn find_opposite_moves(allowed_moves: &[MoveKind]) -> Option<[MoveKind; 2]> {
    let filtered_moves = allowed_moves
        .iter()
        .filter(|m| m.is_side())
        .collect::<Vec<_>>();

    for (i, &m) in filtered_moves.iter().enumerate() {
        for &n in filtered_moves.iter().skip(i) {
            if n.inverse() == *m {
                return Some([*m, *n]);
            }
        }
    }

    None
}

#[derive(Debug)]
struct CornerFinder {
    slots: Vec<Slot>,
    current_moves: Vec<Move>,
    max_depth: u8,
    results: Vec<Commutator>,
}

impl CornerFinder {
    fn search(
        mut self,
        initial_state: FaceletCube,
        interchange_moves: &[Move],
        allowed_moves: &[Move],
    ) -> Vec<Commutator> {
        self.find_interchange(initial_state, interchange_moves, allowed_moves, 0);
        self.results
    }

    fn find_interchange(
        &mut self,
        state: FaceletCube,
        interchange_moves: &[Move],
        allowed_moves: &[Move],
        depth: u8,
    ) {
        if self.max_depth - depth < 4 {
            return;
        }

        for m in interchange_moves {
            let new_state = state.clone().apply_move(*m);

            if let Some((source, target)) = self.check_interchange(&new_state) {
                self.find_insertion(&state, *m, source, target, allowed_moves, depth + 1);
            }
        }

        self.find_setup_moves(state, interchange_moves, allowed_moves, depth + 1);
    }

    fn check_interchange(&self, state: &FaceletCube) -> Option<(Slot, Slot)> {
        for slot in &self.slots {
            let current = state[slot.original];

            if slot.current != current && self.inside_cycle(current) {
                let source = self
                    .slots
                    .iter()
                    .find(|s| s.current != slot.current && s.current != current)
                    .cloned(); // sticker outside the interchange

                return source.map(|source| (source, slot.clone()));
            }
        }

        None
    }

    fn find_insertion(
        &mut self,
        state: &FaceletCube,
        interchange: Move,
        source: Slot,
        target: Slot,
        allowed_moves: &[Move],
        depth: u8,
    ) {
        if self.max_depth - depth < 3 {
            return;
        }

        let wrapper_moves = allowed_moves.iter().filter(|m| m.kind != interchange.kind);
        let second_moves = interchange.kind.inverse().to_moves();

        for wm in wrapper_moves {
            let first = state.clone().apply_move(*wm);

            for sm in &second_moves {
                let second = first.clone().apply_move(*sm);
                let last = second.apply_move(wm.inverse());

                if last[target.original] == source.current {
                    let insertion = Alg::new([*wm, *sm, wm.inverse()]);
                    self.add_commutator(interchange, insertion, source.clone(), target.clone());
                }
            }
        }
    }

    fn find_setup_moves(
        &mut self,
        state: FaceletCube,
        interchange_moves: &[Move],
        allowed_moves: &[Move],
        depth: u8,
    ) {
        if self.max_depth - depth < 5 {
            return;
        }

        for m in allowed_moves {
            if let Some(last) = self.current_moves.last() {
                if last.kind == m.kind {
                    continue;
                }
            }

            let new_state = state.clone().apply_move(*m);
            let prev_slots = self.update_slots(*m);

            self.current_moves.push(*m);
            self.find_interchange(new_state, interchange_moves, allowed_moves, depth + 1);
            self.current_moves.pop();
            self.slots = prev_slots;
        }
    }

    fn add_commutator(&mut self, interchange: Move, insertion: Alg, source: Slot, target: Slot) {
        let setup = match !self.current_moves.is_empty() {
            true => Some(Alg::new(self.current_moves.clone())),
            false => None,
        };
        let insertion_first = target.original == source.current;
        let commutator = Commutator {
            setup,
            interchange,
            insertion,
            insertion_first,
        };

        self.results.push(commutator);
    }

    fn update_slots(&mut self, m: Move) -> Vec<Slot> {
        let prev = self.slots.clone();
        let permutation = FaceletPermutation::from(m);

        for slot in self.slots.iter_mut() {
            slot.original = permutation[slot.original];
        }

        prev
    }

    fn inside_cycle(&self, facelet: Facelet) -> bool {
        self.slots.iter().find(|s| s.original == facelet).is_some()
    }
}

pub fn find_edge_commutator(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    todo!()
}
