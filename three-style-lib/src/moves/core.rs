use crate::error::Error;
use std::{fmt, ops::Mul, str::FromStr};

pub trait Inverse {
    fn inverse(&self) -> Self;
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum MoveKind {
    U, R, F, D, L, B,
    X, Y, Z, M, E, S,
    Uw, Rw, Fw, Dw, Lw, Bw,
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

    pub fn parallel(&self) -> Vec<MoveKind> {
        match self {
            MoveKind::E => vec![MoveKind::U, MoveKind::D],
            MoveKind::M => vec![MoveKind::R, MoveKind::L],
            MoveKind::S => vec![MoveKind::F, MoveKind::B],
            MoveKind::U | MoveKind::D => vec![self.inverse(), MoveKind::E],
            MoveKind::R | MoveKind::L => vec![self.inverse(), MoveKind::M],
            MoveKind::F | MoveKind::B => vec![self.inverse(), MoveKind::S],
            _ => vec![self.inverse()],
        }
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

impl FromStr for MoveKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
        }
    }
}

impl fmt::Display for MoveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{self:?}");
        let s = if self.is_wide() { s.to_lowercase() } else { s };
        write!(f, "{}", &s[..1])
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

impl FromStr for MoveCount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(MoveCount::Double),
            "'" => Ok(MoveCount::Prime),
            "" => Ok(MoveCount::Simple),
            _ => Err(Error::InvalidMove(s.to_owned())),
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

    fn reduce(&self, rhs: Move) -> Option<Move> {
        use {MoveCount as C, MoveKind as M};

        let is_inversed = self.count.inverse() == rhs.count;
        let is_slice = self.kind.is_slice();
        let count = if is_slice { rhs.count } else { self.count }; // determined by wide or side moves

        match (self.kind, self.count, rhs.kind, rhs.count) {
            // wide move generators
            (M::U, _, M::E, _) => Some(Move::new(M::Uw, count)),
            (M::E, _, M::D, _) if is_inversed => Some(Move::new(M::Dw, count)),
            (M::L, _, M::M, _) => Some(Move::new(M::Lw, count)),
            (M::M, _, M::R, _) if is_inversed => Some(Move::new(M::Rw, count)),
            (M::F, _, M::S, _) => Some(Move::new(M::Fw, count)),
            (M::S, _, M::B, _) if is_inversed => Some(Move::new(M::Bw, count)),

            // wide move reduction
            (M::R, C::Prime, M::Rw, C::Simple) => Some(Move::new(M::M, C::Prime)),
            (M::R, C::Simple, M::Rw, C::Prime) => Some(Move::new(M::M, C::Simple)),
            (M::L, C::Prime, M::Lw, C::Simple) => Some(Move::new(M::M, C::Simple)),
            (M::L, C::Simple, M::Lw, C::Prime) => Some(Move::new(M::M, C::Prime)),
            (M::U, C::Prime, M::Uw, C::Simple) => Some(Move::new(M::E, C::Simple)),
            (M::U, C::Simple, M::Uw, C::Prime) => Some(Move::new(M::E, C::Prime)),
            (M::D, C::Prime, M::Dw, C::Simple) => Some(Move::new(M::E, C::Prime)),
            (M::D, C::Simple, M::Dw, C::Prime) => Some(Move::new(M::E, C::Simple)),
            (M::F, C::Prime, M::Fw, C::Simple) => Some(Move::new(M::S, C::Simple)),
            (M::F, C::Simple, M::Fw, C::Prime) => Some(Move::new(M::S, C::Prime)),
            (M::B, C::Prime, M::Bw, C::Simple) => Some(Move::new(M::S, C::Prime)),
            (M::B, C::Simple, M::Bw, C::Prime) => Some(Move::new(M::S, C::Simple)),

            (M::M, _, M::Lw, _) if is_inversed => Some(Move::new(M::L, count)),
            (M::M, _, M::Rw, _) if self.count == rhs.count => Some(Move::new(M::R, count)),
            (M::E, _, M::Uw, _) if is_inversed => Some(Move::new(M::U, count)),
            (M::E, _, M::Dw, _) if self.count == rhs.count => Some(Move::new(M::D, count)),
            (M::S, _, M::Fw, _) if is_inversed => Some(Move::new(M::F, count)),
            (M::S, _, M::Bw, _) if self.count == rhs.count => Some(Move::new(M::B, count)),

            // move count reduction
            (_, _, _, _) if self.kind == rhs.kind && !is_inversed => {
                let kind = self.kind;
                let count = match (self.count as usize + rhs.count as usize) % 4 {
                    2 => MoveCount::Double,
                    3 => MoveCount::Prime,
                    _ => MoveCount::Simple,
                };
                Some(Move { kind, count })
            }
            _ => None,
        }
    }
}

impl Inverse for Move {
    fn inverse(&self) -> Self {
        Self::new(self.kind, self.count.inverse())
    }
}

impl Mul<Move> for Move {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        self.reduce(rhs).or(rhs.reduce(self)) // reduce is not commutative
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = s.get(..1).unwrap_or_default();
        let kind = MoveKind::from_str(kind)?;
        let count = s.get(1..2).unwrap_or_default();
        let count = MoveCount::from_str(count)?;

        Ok(Self { kind, count })
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
