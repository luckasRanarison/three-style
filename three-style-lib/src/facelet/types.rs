use self::Facelet as F;
use std::fmt;

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Facelet {
    U0, U1, U2, U3, U4, U5, U6, U7, U8,
    R0, R1, R2, R3, R4, R5, R6, R7, R8,
    F0, F1, F2, F3, F4, F5, F6, F7, F8,
    D0, D1, D2, D3, D4, D5, D6, D7, D8,
    L0, L1, L2, L3, L4, L5, L6, L7, L8,
    B0, B1, B2, B3, B4, B5, B6, B7, B8,
}

impl Facelet {
    pub fn as_color(&self) -> Color {
        match self {
            F::U0 | F::U1 | F::U2 | F::U3 | F::U4 | F::U5 | F::U6 | F::U7 | F::U8 => Color::U,
            F::R0 | F::R1 | F::R2 | F::R3 | F::R4 | F::R5 | F::R6 | F::R7 | F::R8 => Color::R,
            F::F0 | F::F1 | F::F2 | F::F3 | F::F4 | F::F5 | F::F6 | F::F7 | F::F8 => Color::F,
            F::D0 | F::D1 | F::D2 | F::D3 | F::D4 | F::D5 | F::D6 | F::D7 | F::D8 => Color::D,
            F::L0 | F::L1 | F::L2 | F::L3 | F::L4 | F::L5 | F::L6 | F::L7 | F::L8 => Color::L,
            F::B0 | F::B1 | F::B2 | F::B3 | F::B4 | F::B5 | F::B6 | F::B7 | F::B8 => Color::B,
        }
    }
}

impl fmt::Display for Facelet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait FaceletTarget {
    fn to_facelets(&self) -> Vec<Facelet>;

    fn as_facelet(&self) -> Facelet {
        self.to_facelets()[0]
    }
}

/// Represents all the facelets of a cube in the`URFDLB` order.
pub type FaceletState = [Facelet; 54];

#[rustfmt::skip]
pub const DEFAULT_STATE: FaceletState = [
    F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
    F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
    F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
    F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
    F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
    F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
];

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color { U, R, F, D, L, B }

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
