use crate::error::Error;
use std::{fmt, ops::BitAnd, str::FromStr};

pub trait Inverse {
    fn inverse(&self) -> Self;
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveKind {
    U, R, F, D, L, B,
    X, Y, Z, M, E, S,
    Uw, Rw, Fw, Dw, Lw, Bw,
}

impl fmt::Display for MoveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MoveKind {
    pub fn is_side(self) -> bool {
        matches!(
            self,
            Self::U | Self::R | Self::F | Self::D | Self::L | Self::B
        )
    }

    pub fn is_rotation(self) -> bool {
        matches!(self, Self::X | Self::Y | Self::Z)
    }

    pub fn is_slice(self) -> bool {
        matches!(self, Self::M | Self::E | Self::S)
    }

    pub fn is_wide(self) -> bool {
        matches!(
            self,
            Self::Uw | Self::Rw | Self::Fw | Self::Dw | Self::Lw | Self::Bw
        )
    }

    pub fn to_moves(&self) -> [Move; 3] {
        [
            Move::new(*self, MoveCount::Simple),
            Move::new(*self, MoveCount::Double),
            Move::new(*self, MoveCount::Prime),
        ]
    }
}

impl Inverse for MoveKind {
    fn inverse(&self) -> Self {
        match self {
            MoveKind::U => MoveKind::D,
            MoveKind::R => MoveKind::L,
            MoveKind::F => MoveKind::B,
            MoveKind::D => MoveKind::U,
            MoveKind::L => MoveKind::R,
            MoveKind::B => MoveKind::F,
            MoveKind::Uw => MoveKind::Dw,
            MoveKind::Rw => MoveKind::Lw,
            MoveKind::Fw => MoveKind::Bw,
            MoveKind::Dw => MoveKind::Uw,
            MoveKind::Lw => MoveKind::Rw,
            MoveKind::Bw => MoveKind::Fw,
            _ => *self,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveCount {
    Simple = 1,
    Double = 2,
    Prime = 3,
}

impl Inverse for MoveCount {
    fn inverse(&self) -> Self {
        match self {
            MoveCount::Simple => MoveCount::Prime,
            MoveCount::Double => MoveCount::Double,
            MoveCount::Prime => MoveCount::Simple,
        }
    }
}

impl fmt::Display for MoveCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveCount::Simple => write!(f, ""),
            MoveCount::Double => write!(f, "2"),
            MoveCount::Prime => write!(f, "'"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub kind: MoveKind,
    pub count: MoveCount,
}

impl Move {
    pub fn new(kind: MoveKind, count: MoveCount) -> Self {
        Self { kind, count }
    }
}

impl Inverse for Move {
    fn inverse(&self) -> Self {
        Self::new(self.kind, self.count.inverse())
    }
}

impl BitAnd<Move> for Move {
    type Output = Option<Move>;

    fn bitand(self, rhs: Move) -> Self::Output {
        if rhs.kind != self.kind {
            return None;
        }

        if self.count == rhs.count.inverse() {
            return None;
        }

        let kind = self.kind;
        let count = match (self.count as usize + rhs.count as usize) % 4 {
            2 => MoveCount::Double,
            3 => MoveCount::Prime,
            _ => MoveCount::Simple,
        };

        Some(Move { kind, count })
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = match s.get(1..2) {
            Some("2") => MoveCount::Double,
            Some("'") => MoveCount::Prime,
            None => MoveCount::Simple,
            _ => return Err(Error::InvalidMove(s.to_owned())),
        };

        let kind = match &s[0..1] {
            "U" => Ok(MoveKind::U),
            "F" => Ok(MoveKind::F),
            "R" => Ok(MoveKind::R),
            "B" => Ok(MoveKind::B),
            "L" => Ok(MoveKind::L),
            "D" => Ok(MoveKind::D),
            "M" => Ok(MoveKind::M),
            "S" => Ok(MoveKind::S),
            "E" => Ok(MoveKind::E),
            "x" => Ok(MoveKind::X),
            "y" => Ok(MoveKind::Y),
            "z" => Ok(MoveKind::Z),
            "u" => Ok(MoveKind::Uw),
            "f" => Ok(MoveKind::Fw),
            "r" => Ok(MoveKind::Rw),
            "b" => Ok(MoveKind::Bw),
            "l" => Ok(MoveKind::Lw),
            "d" => Ok(MoveKind::Dw),
            _ => Err(Error::InvalidMove(s.to_owned())),
        };

        kind.map(|kind| Self { kind, count })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.kind, self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_str() {
        assert_eq!(
            Move::from_str("R"),
            Ok(Move::new(MoveKind::R, MoveCount::Simple))
        );
        assert_eq!(
            Move::from_str("R2"),
            Ok(Move::new(MoveKind::R, MoveCount::Double))
        );
        assert_eq!(
            Move::from_str("R'"),
            Ok(Move::new(MoveKind::R, MoveCount::Prime))
        );
    }
}
