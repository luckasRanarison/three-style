use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Normal = 1,
    Double = 2,
    Prime = 3,
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveType::Normal => write!(f, ""),
            MoveType::Double => write!(f, "2"),
            MoveType::Prime => write!(f, "'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    U(MoveType),
    F(MoveType),
    R(MoveType),
    B(MoveType),
    L(MoveType),
    D(MoveType),
    M(MoveType),
    S(MoveType),
    E(MoveType),
    X(MoveType),
    Y(MoveType),
    Z(MoveType),
    Uw(MoveType),
    Fw(MoveType),
    Rw(MoveType),
    Bw(MoveType),
    Lw(MoveType),
    Dw(MoveType),
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::U(t) => write!(f, "U{t}"),
            Move::F(t) => write!(f, "F{t}"),
            Move::R(t) => write!(f, "R{t}"),
            Move::B(t) => write!(f, "B{t}"),
            Move::L(t) => write!(f, "L{t}"),
            Move::D(t) => write!(f, "D{t}"),
            Move::M(t) => write!(f, "M{t}"),
            Move::S(t) => write!(f, "S{t}"),
            Move::E(t) => write!(f, "E{t}"),
            Move::X(t) => write!(f, "x{t}"),
            Move::Y(t) => write!(f, "y{t}"),
            Move::Z(t) => write!(f, "z{t}"),
            Move::Uw(t) => write!(f, "u{t}"),
            Move::Fw(t) => write!(f, "f{t}"),
            Move::Rw(t) => write!(f, "r{t}"),
            Move::Bw(t) => write!(f, "b{t}"),
            Move::Lw(t) => write!(f, "l{t}"),
            Move::Dw(t) => write!(f, "d{t}"),
        }
    }
}

impl Move {
    pub fn from_str(s: &str) -> Option<Self> {
        let t = match s.get(1..2) {
            Some("2") => MoveType::Double,
            Some("'") => MoveType::Prime,
            None => MoveType::Normal,
            _ => return None,
        };

        match &s[0..1] {
            "U" => Some(Move::U(t)),
            "F" => Some(Move::F(t)),
            "R" => Some(Move::R(t)),
            "B" => Some(Move::B(t)),
            "L" => Some(Move::L(t)),
            "D" => Some(Move::D(t)),
            "M" => Some(Move::M(t)),
            "S" => Some(Move::S(t)),
            "E" => Some(Move::E(t)),
            "x" => Some(Move::X(t)),
            "y" => Some(Move::Y(t)),
            "z" => Some(Move::Z(t)),
            "u" => Some(Move::Uw(t)),
            "f" => Some(Move::Fw(t)),
            "r" => Some(Move::Rw(t)),
            "b" => Some(Move::Bw(t)),
            "l" => Some(Move::Lw(t)),
            "d" => Some(Move::Dw(t)),
            _ => None,
        }
    }

    pub fn get_type(&self) -> MoveType {
        match self {
            Move::U(t)
            | Move::F(t)
            | Move::R(t)
            | Move::B(t)
            | Move::L(t)
            | Move::D(t)
            | Move::M(t)
            | Move::S(t)
            | Move::E(t)
            | Move::X(t)
            | Move::Y(t)
            | Move::Z(t)
            | Move::Uw(t)
            | Move::Fw(t)
            | Move::Rw(t)
            | Move::Bw(t)
            | Move::Lw(t)
            | Move::Dw(t) => *t,
        }
    }
}

pub fn moves_from_str(s: &str) -> Option<Vec<Move>> {
    s.split_whitespace().map(Move::from_str).collect()
}

#[cfg(test)]
mod tests {
    use crate::moves::{Move, MoveType};

    #[test]
    fn test_move_string() {
        assert_eq!(Move::from_str("R"), Some(Move::R(MoveType::Normal)));
        assert_eq!(Move::from_str("R'"), Some(Move::R(MoveType::Prime)));
        assert_eq!(Move::from_str("R2"), Some(Move::R(MoveType::Double)));

        assert_eq!(&Move::U(MoveType::Normal).to_string(), "U");
        assert_eq!(&Move::U(MoveType::Prime).to_string(), "U'");
        assert_eq!(&Move::U(MoveType::Double).to_string(), "U2");
    }
}
