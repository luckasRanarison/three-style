use super::types::{Commutator, Cycle};
use crate::{
    facelet::{moves::FaceletPermutation, Facelet, FaceletCube, FaceletTarget},
    moves::{Alg, Inverse, Move, MoveCount, MoveKind},
    sticker::{Corner, Edge},
};
use std::collections::HashSet;

#[derive(Debug)]
struct SearchParams {
    state: FaceletCube,
    slots: Vec<Slot>,
    interchange_moves: Vec<Move>,
    allowed_moves: Vec<Move>,
    depth: u8,
}

impl SearchParams {
    fn new<T>(cycle: Cycle<T>, initial_state: FaceletCube, allowed_moves: &[MoveKind]) -> Self
    where
        T: Clone + Copy + FaceletTarget,
    {
        let slots = cycle
            .to_facelets()
            .into_iter()
            .map(|f| Slot {
                initial_position: f,
                current_position: f,
                value: initial_state[f],
            })
            .collect();

        let interchange_moves = find_parallel_moves(allowed_moves)
            .into_iter()
            .flat_map(|m| m.to_moves())
            .collect::<Vec<_>>();

        let allowed_moves = allowed_moves
            .into_iter()
            .flat_map(MoveKind::to_moves)
            .collect::<Vec<_>>();

        Self {
            state: initial_state,
            slots,
            interchange_moves,
            allowed_moves,
            depth: 0,
        }
    }

    fn next(&self, m: Move) -> Self {
        let state = self.state.clone().apply_move(m);
        let permutation = FaceletPermutation::from(m);
        let slots = self
            .slots
            .iter()
            .map(|s| Slot {
                initial_position: s.initial_position,
                current_position: permutation[s.current_position],
                value: s.value,
            })
            .collect();

        Self {
            state,
            slots,
            interchange_moves: self.interchange_moves.clone(),
            allowed_moves: self.allowed_moves.clone(),
            depth: self.depth + 1,
        }
    }

    fn inside_cycle(&self, facelet: Facelet) -> bool {
        self.slots.iter().find(|s| s.value == facelet).is_some()
    }

    fn get_remaining_slot(&self, first: Facelet, second: Facelet) -> Slot {
        self.slots
            .iter()
            .find(|s| s.value != first && s.value != second)
            .cloned()
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Slot {
    initial_position: Facelet,
    current_position: Facelet,
    value: Facelet,
}

#[derive(Debug)]
struct CommutatorFinder {
    current_moves: Vec<Move>,
    max_depth: u8,
    results: Vec<Commutator>,
}

impl CommutatorFinder {
    fn new(max_depth: u8) -> Self {
        Self {
            current_moves: Vec::new(),
            max_depth,
            results: Vec::new(),
        }
    }

    fn search(mut self, params: SearchParams) -> Vec<Commutator> {
        self.find_interchange(params);
        self.results
    }

    fn find_interchange(&mut self, params: SearchParams) {
        if self.max_depth - params.depth < 4 {
            return;
        }

        for m in &params.interchange_moves {
            let new_state = params.state.clone().apply_move(*m);

            if let Some(insertion) = self.check_interchange(&params, &new_state) {
                self.find_insertion(&params, *m, insertion);
            }
        }

        self.find_setup_moves(params);
    }

    fn check_interchange(
        &self,
        params: &SearchParams,
        state: &FaceletCube,
    ) -> Option<(Slot, Slot)> {
        for slot in &params.slots {
            let current = state[slot.current_position];

            if slot.value != current && params.inside_cycle(current) {
                let source = params.get_remaining_slot(slot.value, current);

                if state[source.current_position] == source.value {
                    return Some((source, slot.clone()));
                }
            }
        }

        None
    }

    fn find_insertion(
        &mut self,
        params: &SearchParams,
        interchange: Move,
        insertion: (Slot, Slot),
    ) {
        if self.max_depth - params.depth < 3 {
            return;
        }

        let (source, target) = insertion;
        let wrapper_moves = params
            .allowed_moves
            .iter()
            .filter(|m| m.kind != interchange.kind && m.count != MoveCount::Double);
        let second_moves = interchange
            .kind
            .parallel()
            .into_iter()
            .flat_map(|m| m.to_moves().into_iter())
            .collect::<Vec<_>>();

        for wm in wrapper_moves {
            let first = params.state.clone().apply_move(*wm);

            for sm in &second_moves {
                let second = first.clone().apply_move(*sm);
                let last = second.apply_move(wm.inverse());

                if last[target.current_position] == source.value {
                    let insertion = Alg::new([*wm, *sm, wm.inverse()]);
                    let insertion_first = target.initial_position == source.value;
                    self.add_commutator(interchange, insertion, insertion_first);
                }
            }
        }
    }

    fn find_setup_moves(&mut self, params: SearchParams) {
        for m in &params.allowed_moves {
            if let Some(last) = self.current_moves.last() {
                if last.kind == m.kind {
                    continue;
                }
            }

            self.current_moves.push(*m);
            self.find_interchange(params.next(*m));
            self.current_moves.pop();
        }
    }

    fn add_commutator(&mut self, interchange: Move, insertion: Alg, insertion_first: bool) {
        let setup = match !self.current_moves.is_empty() {
            true => Some(Alg::new(self.current_moves.clone())),
            false => None,
        };
        let commutator = Commutator {
            setup,
            interchange,
            insertion,
            insertion_first,
        };

        self.results.push(commutator);
    }
}

fn find_parallel_moves(allowed_moves: &[MoveKind]) -> HashSet<MoveKind> {
    let mut results = HashSet::new();

    for (i, &m) in allowed_moves.iter().enumerate() {
        for &n in allowed_moves.iter().skip(i + 1) {
            let parallel = m.parallel().into_iter().find(|&p| p == n).is_some();

            if parallel {
                results.insert(m);
                results.insert(n);
            }
        }
    }

    results
}

pub fn find_corner_commutators(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    let initial_state = FaceletCube::try_from(cycle.inverse());

    initial_state
        .map(|state| {
            let finder = CommutatorFinder::new(max_depth);
            let params = SearchParams::new(cycle, state, allowed_moves);

            finder.search(params)
        })
        .unwrap_or_default()
}

pub fn find_edge_commutators(
    cycle: Cycle<Edge>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    let initial_state = FaceletCube::try_from(cycle.inverse());

    initial_state
        .map(|state| {
            let finder = CommutatorFinder::new(max_depth);
            let params = SearchParams::new(cycle, state, allowed_moves);

            finder.search(params)
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alg;

    #[test]
    fn test_parallel_moves() {
        let moves = [
            MoveKind::U,
            MoveKind::F,
            MoveKind::R,
            MoveKind::D,
            MoveKind::L,
        ];
        let results = find_parallel_moves(&moves);
        let expected = [MoveKind::U, MoveKind::D, MoveKind::R, MoveKind::L]
            .into_iter()
            .collect::<HashSet<_>>();

        assert_eq!(expected, results);

        let moves = [
            MoveKind::L,
            MoveKind::F,
            MoveKind::U,
            MoveKind::R,
            MoveKind::M,
        ];
        let results = find_parallel_moves(&moves);
        let expected = vec![MoveKind::L, MoveKind::R, MoveKind::M]
            .into_iter()
            .collect::<HashSet<_>>();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_corner_commutators() {
        let cycle = Cycle::new(Corner::UFR, Corner::URB, Corner::RFD);
        let allowed_moves = vec![MoveKind::U, MoveKind::R, MoveKind::D];
        let results = find_corner_commutators(cycle, &allowed_moves, 4);
        let expected = Commutator {
            setup: None,
            interchange: Move::new(MoveKind::U, MoveCount::Simple),
            insertion: alg!("R' D' R"),
            insertion_first: true,
        };

        assert!(results.len() == 1);
        assert_eq!(results[0], expected);
    }
}
