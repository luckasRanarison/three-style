use crate::error::Error;
use std::{fmt, ops::Mul, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Normal = 1,
    Double = 2,
    Prime = 3,
}

impl MoveDirection {
    pub fn inverse(&self) -> Self {
        match self {
            MoveDirection::Normal => MoveDirection::Prime,
            MoveDirection::Prime => MoveDirection::Normal,
            _ => *self,
        }
    }
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

impl MoveKind {
    pub fn inverse(&self) -> Self {
        match self {
            MoveKind::U => MoveKind::D,
            MoveKind::F => MoveKind::B,
            MoveKind::R => MoveKind::L,
            MoveKind::B => MoveKind::F,
            MoveKind::L => MoveKind::R,
            MoveKind::D => MoveKind::U,
            MoveKind::Uw => MoveKind::Dw,
            MoveKind::Fw => MoveKind::Bw,
            MoveKind::Rw => MoveKind::Lw,
            MoveKind::Bw => MoveKind::Fw,
            MoveKind::Lw => MoveKind::Rw,
            MoveKind::Dw => MoveKind::Uw,
            _ => *self,
        }
    }

    pub fn get_move_variants(&self) -> Vec<Move> {
        vec![
            Move::new(*self, MoveDirection::Normal),
            Move::new(*self, MoveDirection::Prime),
            Move::new(*self, MoveDirection::Double),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub kind: MoveKind,
    pub direction: MoveDirection,
}

impl Move {
    fn new(kind: MoveKind, direction: MoveDirection) -> Self {
        Move { kind, direction }
    }

    pub fn inverse(&self) -> Self {
        let kind = self.kind;
        let direction = self.direction.inverse();

        Move::new(kind, direction)
    }
}

impl Mul<Move> for Move {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        if rhs.kind != self.kind {
            return None;
        }

        if self.direction == rhs.direction.inverse() {
            return None;
        }

        let kind = self.kind;
        let count = self.direction as usize + rhs.direction as usize;
        let direction = match count % 4 {
            2 => MoveDirection::Double,
            3 => MoveDirection::Prime,
            _ => MoveDirection::Normal,
        };

        Some(Move { kind, direction })
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

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.kind, self.direction)
    }
}

pub fn moves_from_str(s: &str) -> Result<Vec<Move>, Error> {
    s.split_whitespace().map(Move::from_str).collect()
}

pub fn format_moves(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn reverse_moves(moves: &[Move]) -> Vec<Move> {
    moves.iter().rev().map(|m| m.inverse()).collect()
}

pub fn clean_moves(moves: Vec<Move>) -> Vec<Move> {
    let mut primary: Option<Move> = None;
    let mut secondary: Option<Move> = None;
    let mut result: Vec<Move> = Vec::new();

    for m in moves {
        let p = match primary {
            Some(p) => p,
            None => {
                primary = Some(m);
                continue;
            }
        };

        if m.kind == p.kind {
            primary = p * m;
        } else if m.kind == p.kind.inverse() {
            if let Some(s) = secondary {
                secondary = s * m;
            } else {
                secondary = Some(m);
            }
        } else {
            if let Some(p) = primary.take() {
                result.push(p);
            }
            if let Some(s) = secondary.take() {
                result.push(s);
            }
            primary = Some(m);
        }
    }

    if let Some(p) = primary.take() {
        result.push(p);
    }
    if let Some(s) = secondary.take() {
        result.push(s);
    }

    result
}

pub fn find_parallel_moves(moves: &[MoveKind]) -> Option<(MoveKind, MoveKind)> {
    for (i, m) in moves.iter().enumerate() {
        for n in &moves[i..] {
            if m.inverse() == *n {
                return Some((*m, *n));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::moves_from_str;
    use crate::moves::{clean_moves, Move, MoveDirection, MoveKind};
    use std::str::FromStr;

    #[test]
    fn test_move_string() {
        assert_eq!(
            Move::from_str("R"),
            Ok(Move::new(MoveKind::R, MoveDirection::Normal))
        );
        assert_eq!(
            Move::from_str("R'"),
            Ok(Move::new(MoveKind::R, MoveDirection::Prime))
        );
        assert_eq!(
            Move::from_str("R2"),
            Ok(Move::new(MoveKind::R, MoveDirection::Double))
        );

        assert_eq!(
            &Move::new(MoveKind::U, MoveDirection::Normal).to_string(),
            "U"
        );
        assert_eq!(
            &Move::new(MoveKind::U, MoveDirection::Prime).to_string(),
            "U'"
        );
        assert_eq!(
            &Move::new(MoveKind::U, MoveDirection::Double).to_string(),
            "U2"
        );
    }

    #[test]
    fn test_move_cleanup() {
        let raw = moves_from_str("R U U").unwrap();
        let expected = moves_from_str("R U2").unwrap();

        assert_eq!(expected, clean_moves(raw));

        let raw = moves_from_str("R U2 U").unwrap();
        let expected = moves_from_str("R U'").unwrap();

        assert_eq!(expected, clean_moves(raw));

        let raw = moves_from_str("R U2 U2").unwrap();
        let expected = moves_from_str("R").unwrap();

        assert_eq!(expected, clean_moves(raw));

        let raw = moves_from_str("R U U'").unwrap();
        let expected = moves_from_str("R").unwrap();

        assert_eq!(expected, clean_moves(raw));

        let raw = moves_from_str("R U D U'").unwrap();
        let expected = moves_from_str("R D").unwrap();

        assert_eq!(expected, clean_moves(raw));
    }
}
