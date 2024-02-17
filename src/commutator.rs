use crate::{
    error::Error,
    facelet::{Facelet, FaceletTarget},
    moves::{clean_moves, format_moves, reverse_moves, Move},
    state::Cube,
    sticker::{Corner, Edge},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Cycle<T>
where
    T: Clone + Copy + FaceletTarget,
{
    pub first: T,
    pub second: T,
    pub third: T,
}

impl<T> Cycle<T>
where
    T: Clone + Copy + FaceletTarget,
{
    pub fn new(first: T, second: T, third: T) -> Self {
        Self {
            first,
            second,
            third,
        }
    }

    pub fn alt(&self) -> Self {
        Self {
            first: self.first,
            second: self.third,
            third: self.second,
        }
    }

    pub fn to_facelets(&self) -> [Facelet; 3] {
        [
            self.first.as_facelet(),
            self.second.as_facelet(),
            self.third.as_facelet(),
        ]
    }
}

pub trait ThreeCycle: Sized {
    fn edge_cycle(self, cycle: Cycle<Edge>) -> Result<Self, Error>;
    fn corner_cycle(self, cycle: Cycle<Corner>) -> Result<Self, Error>;
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

    pub fn solve(&self, state: Cube) -> bool {
        state.apply_moves(&self.expand()).is_solved()
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
