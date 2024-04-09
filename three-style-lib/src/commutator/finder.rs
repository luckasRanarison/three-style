use super::types::{Commutator, Cycle};
use crate::{
    facelet::{moves::FaceletPermutation, Facelet, FaceletCube, FaceletTarget},
    moves::{Alg, Inverse, Move, MoveCount, MoveKind},
    sticker::{Corner, Edge},
};
use std::{fmt, ops::Not};

#[derive(Debug, PartialEq, Clone)]
struct Slot {
    initial_position: Facelet,
    current_position: Facelet,
    value: Facelet,
}

#[derive(Debug)]
struct Insertion {
    source: Slot,
    target: Slot,
}

#[derive(Debug)]
struct SearchParams<'a> {
    state: FaceletCube,
    slots: [Slot; 3],
    allowed_moves: &'a [Move],
    depth: u8,
}

impl<'a> SearchParams<'a> {
    fn new<T>(cycle: Cycle<T>, state: FaceletCube, allowed_moves: &'a [Move]) -> Self
    where
        T: Clone + Copy + FaceletTarget,
    {
        let slots = cycle.to_facelets().map(|f| Slot {
            initial_position: f,
            current_position: f,
            value: state[f],
        });

        Self {
            state,
            slots,
            allowed_moves,
            depth: 0,
        }
    }

    fn next(&self, m: Move) -> Self {
        let state = self.state.apply_move(m);
        let permutation = FaceletPermutation::from(m);
        let slots = self.slots.clone().map(|s| Slot {
            initial_position: s.initial_position,
            current_position: permutation[s.current_position],
            value: s.value,
        });

        Self {
            state,
            slots,
            allowed_moves: self.allowed_moves,
            depth: self.depth + 1,
        }
    }

    fn inside_cycle(&self, facelet: Facelet) -> bool {
        self.slots.iter().any(|s| s.value == facelet)
    }

    fn get_remaining_slot(&self, first: Facelet, second: Facelet) -> Slot {
        self.slots
            .iter()
            .find(|s| s.value != first && s.value != second)
            .cloned()
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
enum SearchType {
    Edge,
    Corner,
}

#[derive(Debug)]
struct CommutatorFinder {
    current_moves: Vec<Move>,
    results: Vec<Commutator>,
    search_type: SearchType,
    max_depth: u8,
}

impl CommutatorFinder {
    fn new(max_depth: u8, search_type: SearchType) -> Self {
        Self {
            current_moves: Vec::new(),
            results: Vec::new(),
            search_type,
            max_depth,
        }
    }

    fn search(mut self, params: SearchParams) -> Vec<Commutator> {
        self.find_interchange(params);
        self.results
    }

    fn find_interchange(&mut self, params: SearchParams) {
        let threshold = match self.search_type {
            SearchType::Corner => 4,
            SearchType::Edge => 2,
        };

        if self.max_depth - params.depth < threshold {
            return;
        }

        for &interchange in params.allowed_moves {
            let new_state = params.state.apply_move(interchange);

            if let Some(insertion) = self.check_interchange(&params, &new_state) {
                if self.search_type == SearchType::Edge && interchange.count == MoveCount::Double {
                    self.find_four_mover(&params, interchange, insertion.source.clone());
                }

                if self.max_depth - params.depth > 3 {
                    self.find_insertion(&params, interchange, insertion);
                }
            }
        }

        self.find_setup_moves(params);
    }

    fn check_interchange(&self, params: &SearchParams, state: &FaceletCube) -> Option<Insertion> {
        for slot in &params.slots {
            let current = state[slot.current_position];

            if slot.value != current && params.inside_cycle(current) {
                let source = params.get_remaining_slot(slot.value, current);

                if state[source.current_position] == source.value {
                    return Some(Insertion {
                        source,
                        target: slot.clone(),
                    });
                }
            }
        }

        None
    }

    fn find_insertion(&mut self, params: &SearchParams, interchange: Move, insertion: Insertion) {
        let Insertion { source, target } = insertion;

        let wrapper_moves = params
            .allowed_moves
            .iter()
            .filter(|m| m.kind != interchange.kind && m.count != MoveCount::Double);
        let second_moves = interchange
            .kind
            .parallel()
            .into_iter()
            .flat_map(|m| m.to_moves())
            .filter(|m| params.allowed_moves.contains(m))
            .collect::<Vec<_>>();

        for wm in wrapper_moves {
            let first = params.state.apply_move(*wm);

            for sm in &second_moves {
                let second = first.apply_move(*sm);
                let last = second.apply_move(wm.inverse());

                if last[target.current_position] == source.value {
                    let insertion = Alg::new([*wm, *sm, wm.inverse()]);
                    let insertion_first = target.initial_position == source.value;
                    self.add_commutator(interchange, insertion, insertion_first);
                }
            }
        }
    }

    fn find_four_mover(&mut self, params: &SearchParams, interchange: Move, source: Slot) {
        let slice_moves = params
            .allowed_moves
            .iter()
            .filter(|m| m.kind.is_slice() && m.count != MoveCount::Double);

        for sm in slice_moves {
            let alg = Alg::new([*sm, interchange, sm.inverse()]);
            let state = params.state.apply_alg(&alg);

            for slot in &params.slots {
                if *slot != source && state[slot.current_position] == source.value {
                    let insertion = Alg::new([*sm]);
                    let insertion_first = slot.initial_position != source.value;
                    self.add_commutator(interchange, insertion, insertion_first);
                }
            }
        }
    }

    fn find_setup_moves(&mut self, params: SearchParams) {
        for &m in params.allowed_moves {
            if let Some(last) = self.current_moves.last() {
                if last.kind == m.kind {
                    continue;
                }
            }

            self.current_moves.push(m);
            self.find_interchange(params.next(m));
            self.current_moves.pop();
        }
    }

    fn add_commutator(&mut self, interchange: Move, insertion: Alg, insertion_first: bool) {
        let setup = self
            .current_moves
            .is_empty()
            .not()
            .then_some(Alg::new(self.current_moves.clone()).clean());
        let commutator = Commutator {
            setup,
            interchange,
            insertion,
            insertion_first,
        };

        self.results.push(commutator);
    }
}

fn find_commutators<T>(
    cycle: Cycle<T>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
    search_type: SearchType,
) -> Vec<Commutator>
where
    T: Clone + Copy + FaceletTarget + fmt::Display,
{
    let initial_state = FaceletCube::try_from(cycle.inverse());

    if let Ok(state) = initial_state {
        let allowed_moves = allowed_moves
            .iter()
            .flat_map(MoveKind::to_moves)
            .collect::<Vec<_>>();
        let finder = CommutatorFinder::new(max_depth, search_type);
        let params = SearchParams::new(cycle, state, &allowed_moves);

        finder.search(params)
    } else {
        Vec::new()
    }
}

pub fn find_corner_commutators(
    cycle: Cycle<Corner>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    find_commutators(cycle, allowed_moves, max_depth, SearchType::Corner)
}

pub fn find_edge_commutators(
    cycle: Cycle<Edge>,
    allowed_moves: &[MoveKind],
    max_depth: u8,
) -> Vec<Commutator> {
    find_commutators(cycle, allowed_moves, max_depth, SearchType::Edge)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_commutators(initial_state: FaceletCube, commutator: Vec<Commutator>) {
        assert!(!commutator.is_empty());
        assert!(commutator
            .into_iter()
            .map(|c| initial_state.apply_commutator(&c))
            .all(|s| s == FaceletCube::default()));
    }

    #[test]
    fn test_corner_commutators() {
        let cycle = Cycle::new(Corner::UFR, Corner::URB, Corner::RFD);
        let initial_state = FaceletCube::try_from(cycle.clone().inverse()).unwrap();
        let allowed_moves = vec![MoveKind::U, MoveKind::R, MoveKind::D];
        let results = find_corner_commutators(cycle, &allowed_moves, 6);

        assert_commutators(initial_state, results);
    }

    #[test]
    fn test_edge_commutators() {
        let cycle = Cycle::new(Edge::UF, Edge::UB, Edge::LF);
        let initial_state = FaceletCube::try_from(cycle.clone().inverse()).unwrap();
        let allowed_moves = vec![MoveKind::U, MoveKind::R, MoveKind::E];
        let results = find_edge_commutators(cycle, &allowed_moves, 5);

        assert_commutators(initial_state, results);
    }

    #[test]
    fn test_four_mover() {
        let cycle = Cycle::new(Edge::UF, Edge::UB, Edge::DF);
        let initial_state = FaceletCube::try_from(cycle.clone().inverse()).unwrap();
        let allowed_moves = vec![MoveKind::U, MoveKind::M];
        let results = find_edge_commutators(cycle, &allowed_moves, 2);

        assert_commutators(initial_state, results);
    }
}
