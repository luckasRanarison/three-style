use crate::error::Error;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Normal = 1,
    Double = 2,
    Prime = 3,
}

impl fmt::Display for MoveDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveDirection::Normal => write!(f, ""),
            MoveDirection::Double => write!(f, "2"),
            MoveDirection::Prime => write!(f, "'"),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveKind {
    U, F, R, B, L, D,
    M, S, E, X, Y, Z,
    Uw, Fw, Rw, Bw, Lw, Dw,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub kind: MoveKind,
    pub direction: MoveDirection,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.kind, self.direction)
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.get(1..2) {
            Some("2") => Some(MoveDirection::Double),
            Some("'") => Some(MoveDirection::Prime),
            None => Some(MoveDirection::Normal),
            _ => None,
        };

        let kind = match &s[0..1] {
            "U" => Some(MoveKind::U),
            "F" => Some(MoveKind::F),
            "R" => Some(MoveKind::R),
            "B" => Some(MoveKind::B),
            "L" => Some(MoveKind::L),
            "D" => Some(MoveKind::D),
            "M" => Some(MoveKind::M),
            "S" => Some(MoveKind::S),
            "E" => Some(MoveKind::E),
            "x" => Some(MoveKind::X),
            "y" => Some(MoveKind::Y),
            "z" => Some(MoveKind::Z),
            "u" => Some(MoveKind::Uw),
            "f" => Some(MoveKind::Fw),
            "r" => Some(MoveKind::Rw),
            "b" => Some(MoveKind::Bw),
            "l" => Some(MoveKind::Lw),
            "d" => Some(MoveKind::Dw),
            _ => None,
        };

        kind.zip(direction)
            .ok_or(Error::InvalidMoveString(s.to_owned()))
            .map(|(kind, direction)| Move { kind, direction })
    }
}

pub fn moves_from_str(s: &str) -> Result<Vec<Move>, Error> {
    s.split_whitespace().map(Move::from_str).collect()
}

#[cfg(test)]
mod tests {
    use crate::moves::{Move, MoveDirection, MoveKind};
    use std::str::FromStr;

    #[test]
    fn test_move_string() {
        assert_eq!(
            Move::from_str("R"),
            Ok(Move {
                kind: MoveKind::R,
                direction: MoveDirection::Normal
            })
        );
        assert_eq!(
            Move::from_str("R'"),
            Ok(Move {
                kind: MoveKind::R,
                direction: MoveDirection::Prime
            })
        );
        assert_eq!(
            Move::from_str("R2"),
            Ok(Move {
                kind: MoveKind::R,
                direction: MoveDirection::Double
            })
        );

        assert_eq!(
            &Move {
                kind: MoveKind::U,
                direction: MoveDirection::Normal
            }
            .to_string(),
            "U"
        );
        assert_eq!(
            &Move {
                kind: MoveKind::U,
                direction: MoveDirection::Prime
            }
            .to_string(),
            "U'"
        );
        assert_eq!(
            &Move {
                kind: MoveKind::U,
                direction: MoveDirection::Double
            }
            .to_string(),
            "U2"
        );
    }
}
