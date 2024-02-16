use crate::{
    error::Error,
    moves::{clean_moves, format_moves, reverse_moves, Move},
    sticker::{Corner, Edge},
};
use std::fmt;

pub trait ThreeCycle: Sized {
    fn edge_cycle(self, first: Edge, second: Edge, third: Edge) -> Result<Self, Error>;
    fn corner_cycle(self, first: Corner, second: Corner, third: Corner) -> Result<Self, Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Commutator {
    pub setup: Option<Vec<Move>>,
    pub interchange: Move,
    pub insertion: Vec<Move>,
    pub insertion_first: bool,
}

impl Commutator {
    pub fn is_pure(&self) -> bool {
        self.setup.is_none()
    }

    pub fn expand(&self) -> Vec<Move> {
        let (setup, undo_setup) = self
            .setup
            .as_ref()
            .map(|s| (s.iter(), reverse_moves(s)))
            .unwrap_or_default();
        let interchange = vec![self.interchange];
        let (first, second) = if self.insertion_first {
            (&self.insertion, &interchange)
        } else {
            (&interchange, &self.insertion)
        };

        let raw_moves: Vec<Move> = setup
            .chain(first.iter())
            .chain(second.iter())
            .chain(reverse_moves(first).iter())
            .chain(reverse_moves(second).iter())
            .chain(undo_setup.iter())
            .cloned()
            .collect();

        clean_moves(raw_moves)
    }
}

impl fmt::Display for Commutator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let insertion = format_moves(&self.insertion);
        let interchange = self.interchange.to_string();
        let (first, second) = if self.insertion_first {
            (insertion, interchange)
        } else {
            (interchange, insertion)
        };
        let start = self
            .setup
            .as_ref()
            .map(|s| format!("[{}:", format_moves(s)))
            .unwrap_or_default();
        let end = if self.setup.is_some() { "]" } else { "" };

        write!(f, "{start}[{first}, {second}]{end}")
    }
}
