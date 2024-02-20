use super::{core::Inverse, Move};
use crate::error::Error;
use std::{fmt, ops::Add, str::FromStr};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Alg {
    moves: Vec<Move>,
}

impl Alg {
    pub fn new<T>(moves: T) -> Self
    where
        T: IntoIterator<Item = Move>,
    {
        Self {
            moves: moves.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn clean(&mut self) {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }
}

impl Inverse for Alg {
    fn inverse(&self) -> Self {
        Self {
            moves: self.moves.iter().rev().map(Move::inverse).collect(),
        }
    }
}

impl IntoIterator for Alg {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}

impl FromStr for Alg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .split_whitespace()
            .map(Move::from_str)
            .collect::<Result<Vec<_>, _>>();

        moves.map(Self::new)
    }
}

impl Add<Self> for Alg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            moves: self
                .moves
                .into_iter()
                .chain(rhs.moves.into_iter())
                .collect(),
        }
    }
}

impl Add<&Alg> for &Alg {
    type Output = Alg;

    fn add(self, rhs: &Alg) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl Add<&Alg> for Alg {
    type Output = Self;

    fn add(self, rhs: &Alg) -> Self::Output {
        self + rhs.clone()
    }
}

impl fmt::Display for Alg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .moves
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{s}")
    }
}

#[macro_export]
macro_rules! alg {
    ($moves: expr) => {{
        use std::str::FromStr;
        use $crate::moves::Alg;
        Alg::from_str($moves).unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moves::{MoveCount, MoveKind};

    #[test]
    fn test_basics() {
        let alg = alg!("R U R' U'");
        let inverse = alg.inverse();

        assert_eq!(
            alg,
            Alg {
                moves: vec![
                    Move::new(MoveKind::R, MoveCount::Simple),
                    Move::new(MoveKind::U, MoveCount::Simple),
                    Move::new(MoveKind::R, MoveCount::Prime),
                    Move::new(MoveKind::U, MoveCount::Prime)
                ]
            }
        );

        assert_eq!(
            inverse,
            Alg {
                moves: vec![
                    Move::new(MoveKind::U, MoveCount::Simple),
                    Move::new(MoveKind::R, MoveCount::Simple),
                    Move::new(MoveKind::U, MoveCount::Prime),
                    Move::new(MoveKind::R, MoveCount::Prime)
                ]
            }
        );
    }
}
