use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Normal,
    Double,
    Prime,
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    U(MoveType),
    F(MoveType),
    R(MoveType),
    B(MoveType),
    L(MoveType),
    D(MoveType),
    M(MoveType),
    E(MoveType),
    S(MoveType),
    u(MoveType),
    f(MoveType),
    r(MoveType),
    b(MoveType),
    l(MoveType),
    d(MoveType),
    x(MoveType),
    y(MoveType),
    z(MoveType),
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
            Move::E(t) => write!(f, "E{t}"),
            Move::S(t) => write!(f, "S{t}"),
            Move::u(t) => write!(f, "u{t}"),
            Move::f(t) => write!(f, "f{t}"),
            Move::r(t) => write!(f, "r{t}"),
            Move::b(t) => write!(f, "b{t}"),
            Move::l(t) => write!(f, "l{t}"),
            Move::d(t) => write!(f, "d{t}"),
            Move::x(t) => write!(f, "x{t}"),
            Move::y(t) => write!(f, "y{t}"),
            Move::z(t) => write!(f, "z{t}"),
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
            "E" => Some(Move::E(t)),
            "S" => Some(Move::S(t)),
            "u" => Some(Move::u(t)),
            "f" => Some(Move::f(t)),
            "r" => Some(Move::r(t)),
            "b" => Some(Move::b(t)),
            "l" => Some(Move::l(t)),
            "d" => Some(Move::d(t)),
            "x" => Some(Move::x(t)),
            "y" => Some(Move::y(t)),
            "z" => Some(Move::z(t)),
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
            | Move::E(t)
            | Move::S(t)
            | Move::u(t)
            | Move::f(t)
            | Move::r(t)
            | Move::b(t)
            | Move::l(t)
            | Move::d(t)
            | Move::x(t)
            | Move::y(t)
            | Move::z(t) => *t,
        }
    }

    pub fn derivate(&self, t: MoveType) -> Self {
        match self {
            Move::U(_) => Move::U(t),
            Move::F(_) => Move::F(t),
            Move::R(_) => Move::R(t),
            Move::B(_) => Move::B(t),
            Move::L(_) => Move::L(t),
            Move::D(_) => Move::D(t),
            Move::M(_) => Move::M(t),
            Move::E(_) => Move::E(t),
            Move::S(_) => Move::S(t),
            Move::u(_) => Move::u(t),
            Move::f(_) => Move::f(t),
            Move::r(_) => Move::r(t),
            Move::b(_) => Move::b(t),
            Move::l(_) => Move::l(t),
            Move::d(_) => Move::d(t),
            Move::x(_) => Move::x(t),
            Move::y(_) => Move::y(t),
            Move::z(_) => Move::z(t),
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
