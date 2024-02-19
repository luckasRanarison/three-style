use super::{
    types::{Facelet as F, FaceletState, DEFAULT_STATE},
    FaceletTarget,
};
use crate::{
    commutator::types::{Cycle, ThreeCycle},
    error::Error,
    moves::{Alg, Move, MoveCount, MoveKind},
    sticker::{Corner, Edge},
};
use constants::*;
use std::{
    collections::HashSet,
    fmt,
    ops::{Index, IndexMut, Mul},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FaceletCube(FaceletState);

impl Default for FaceletCube {
    fn default() -> Self {
        Self(DEFAULT_STATE)
    }
}

impl FaceletCube {
    pub fn new(state: FaceletState) -> Self {
        Self(state)
    }

    pub fn is_solved(&self) -> bool {
        self.0
            .iter()
            .map(F::as_color)
            .collect::<Vec<_>>()
            .chunks(9)
            .all(|side| side.iter().all(|c| Some(c) == side.first()))
    }

    pub fn apply_move(self, m: Move) -> Self {
        self * FaceletCube::from(m)
    }

    pub fn apply_alg(self, alg: &Alg) -> Self {
        alg.iter().fold(self, |acc, m| acc.apply_move(*m))
    }
}

impl From<Move> for FaceletCube {
    fn from(value: Move) -> Self {
        let state = match value.kind {
            MoveKind::U => U_CUBE,
            MoveKind::F => F_CUBE,
            MoveKind::R => R_CUBE,
            MoveKind::B => B_CUBE,
            MoveKind::L => L_CUBE,
            MoveKind::D => D_CUBE,
            MoveKind::M => M_CUBE,
            MoveKind::S => S_CUBE,
            MoveKind::E => E_CUBE,
            MoveKind::X => X_CUBE,
            MoveKind::Y => Y_CUBE,
            MoveKind::Z => Z_CUBE,
            MoveKind::Fw => FW_CUBE,
            MoveKind::Lw => LW_CUBE,
            MoveKind::Dw => DW_CUBE,
            MoveKind::Uw => UW_CUBE,
            MoveKind::Rw => RW_CUBE,
            MoveKind::Bw => BW_CUBE,
        };

        match value.count {
            MoveCount::Simple => state,
            MoveCount::Double => state.clone() * state,
            MoveCount::Prime => state.clone() * state.clone() * state,
        }
    }
}

impl TryFrom<Cycle<Corner>> for FaceletCube {
    type Error = Error;

    fn try_from(value: Cycle<Corner>) -> Result<Self, Self::Error> {
        FaceletCube::default().corner_cycle(value)
    }
}

impl TryFrom<Cycle<Edge>> for FaceletCube {
    type Error = Error;

    fn try_from(value: Cycle<Edge>) -> Result<Self, Self::Error> {
        FaceletCube::default().edge_cycle(value)
    }
}

impl IntoIterator for FaceletCube {
    type Item = F;
    type IntoIter = std::array::IntoIter<F, 54>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for FaceletCube {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for FaceletCube {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<F> for FaceletCube {
    type Output = F;

    fn index(&self, index: F) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<F> for FaceletCube {
    fn index_mut(&mut self, index: F) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Mul<Self> for FaceletCube {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = FaceletCube::default();

        for (i, f) in rhs.into_iter().enumerate() {
            res[i] = self[f];
        }

        res
    }
}

impl fmt::Display for FaceletCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|f| f.as_color().to_string())
            .collect::<String>();

        write!(f, "{s}")
    }
}

fn count_unique_facelets(first: &[F], second: &[F], third: &[F]) -> usize {
    first
        .iter()
        .chain(second.iter())
        .chain(third.iter())
        .collect::<HashSet<_>>()
        .len()
}

impl ThreeCycle for FaceletCube {
    fn edge_cycle(self, cycle: Cycle<Edge>) -> Result<Self, Error> {
        let mut res = self.clone();
        let first = cycle.first().to_facelets();
        let second = cycle.second().to_facelets();
        let third = cycle.third().to_facelets();
        let count = count_unique_facelets(&first, &second, &third);

        if count == 6 {
            res.0[first[0] as usize] = self.0[third[0] as usize];
            res.0[first[1] as usize] = self.0[third[1] as usize];
            res.0[second[0] as usize] = self.0[first[0] as usize];
            res.0[second[1] as usize] = self.0[first[1] as usize];
            res.0[third[0] as usize] = self.0[second[0] as usize];
            res.0[third[1] as usize] = self.0[second[1] as usize];

            Ok(res)
        } else {
            Err(Error::InvalidEdgeCycle(cycle.to_owned()))
        }
    }

    fn corner_cycle(self, cycle: Cycle<Corner>) -> Result<Self, Error> {
        let mut res = self.clone();
        let first = cycle.first().to_facelets();
        let second = cycle.second().to_facelets();
        let third = cycle.third().to_facelets();
        let count = count_unique_facelets(&first, &second, &third);

        if count == 9 {
            res.0[first[0] as usize] = self.0[third[0] as usize];
            res.0[first[1] as usize] = self.0[third[1] as usize];
            res.0[first[2] as usize] = self.0[third[2] as usize];
            res.0[second[0] as usize] = self.0[first[0] as usize];
            res.0[second[1] as usize] = self.0[first[1] as usize];
            res.0[second[2] as usize] = self.0[first[2] as usize];
            res.0[third[0] as usize] = self.0[second[0] as usize];
            res.0[third[1] as usize] = self.0[second[1] as usize];
            res.0[third[2] as usize] = self.0[second[2] as usize];

            Ok(res)
        } else {
            Err(Error::InvalidCornerCycle(cycle.to_owned()))
        }
    }
}

#[rustfmt::skip]
mod constants {
    use super::*;

    pub const U_CUBE: FaceletCube = FaceletCube([
        F::U6, F::U3, F::U0, F::U7, F::U4, F::U1, F::U8, F::U5, F::U2,
        F::B0, F::B1, F::B2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
        F::R0, F::R1, F::R2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
        F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        F::F0, F::F1, F::F2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
        F::L0, F::L1, F::L2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    ]);

    pub const R_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::F2, F::U3, F::U4, F::F5, F::U6, F::U7, F::F8,
        F::R6, F::R3, F::R0, F::R7, F::R4, F::R1, F::R8, F::R5, F::R2,
        F::F0, F::F1, F::D2, F::F3, F::F4, F::D5, F::F6, F::F7, F::D8,
        F::D0, F::D1, F::B6, F::D3, F::D4, F::B3, F::D6, F::D7, F::B0,
        F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
        F::U8, F::B1, F::B2, F::U5, F::B4, F::B5, F::U2, F::B7, F::B8,
    ]);

    pub const F_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::L8, F::L5, F::L2,
        F::U6, F::R1, F::R2, F::U7, F::R4, F::R5, F::U8, F::R7, F::R8,
        F::F6, F::F3, F::F0, F::F7, F::F4, F::F1, F::F8, F::F5, F::F2,
        F::R6, F::R3, F::R0, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        F::L0, F::L1, F::D0, F::L3, F::L4, F::D1, F::L6, F::L7, F::D2,
        F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    ]);

    pub const D_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
        F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::F6, F::F7, F::F8,
        F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::L6, F::L7, F::L8,
        F::D6, F::D3, F::D0, F::D7, F::D4, F::D1, F::D8, F::D5, F::D2,
        F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::B6, F::B7, F::B8,
        F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::R6, F::R7, F::R8,
    ]);

    pub const L_CUBE: FaceletCube = FaceletCube([
        F::B8, F::U1, F::U2, F::B5, F::U4, F::U5, F::B2, F::U7, F::U8,
        F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
        F::U0, F::F1, F::F2, F::U3, F::F4, F::F5, F::U6, F::F7, F::F8,
        F::F0, F::D1, F::D2, F::F3, F::D4, F::D5, F::F6, F::D7, F::D8,
        F::L6, F::L3, F::L0, F::L7, F::L4, F::L1, F::L8, F::L5, F::L2,
        F::B0, F::B1, F::D6, F::B3, F::B4, F::D3, F::B6, F::B7, F::D0,
    ]);

    pub const B_CUBE: FaceletCube = FaceletCube([
        F::R2, F::R5, F::R8, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
        F::R0, F::R1, F::D8, F::R3, F::R4, F::D7, F::R6, F::R7, F::D6,
        F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
        F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::L0, F::L3, F::L6,
        F::U2, F::L1, F::L2, F::U1, F::L4, F::L5, F::U0, F::L7, F::L8,
        F::B6, F::B3, F::B0, F::B7, F::B4, F::B1, F::B8, F::B5, F::B2,
    ]);

    pub const M_CUBE: FaceletCube = FaceletCube([
        F::U0, F::B7, F::U2, F::U3, F::B4, F::U5, F::U6, F::B1, F::U8,
        F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
        F::F0, F::U1, F::F2, F::F3, F::U4, F::F5, F::F6, F::U7, F::F8,
        F::D0, F::F1, F::D2, F::D3, F::F4, F::D5, F::D6, F::F7, F::D8,
        F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
        F::B0, F::D7, F::B2, F::B3, F::D4, F::B5, F::B6, F::D1, F::B8,
    ]);

    pub const E_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
        F::R0, F::R1, F::R2, F::F3, F::F4, F::F5, F::R6, F::R7, F::R8,
        F::F0, F::F1, F::F2, F::L3, F::L4, F::L5, F::F6, F::F7, F::F8,
        F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        F::L0, F::L1, F::L2, F::B3, F::B4, F::B5, F::L6, F::L7, F::L8,
        F::B0, F::B1, F::B2, F::R3, F::R4, F::R5, F::B6, F::B7, F::B8,
    ]);

    pub const S_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::U2, F::L7, F::L4, F::L1, F::U6, F::U7, F::U8,
        F::R0, F::U3, F::R2, F::R3, F::U4, F::R5, F::R6, F::U5, F::R8,
        F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
        F::D0, F::D1, F::D2, F::R7, F::R4, F::R1, F::D6, F::D7, F::D8,
        F::L0, F::D3, F::L2, F::L3, F::D4, F::L5, F::L6, F::D5, F::L8,
        F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    ]);

    pub const X_CUBE: FaceletCube = FaceletCube([
        F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
        F::R6, F::R3, F::R0, F::R7, F::R4, F::R1, F::R8, F::R5, F::R2,
        F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        F::B8, F::B7, F::B6, F::B5, F::B4, F::B3, F::B2, F::B1, F::B0,
        F::L2, F::L5, F::L8, F::L1, F::L4, F::L7, F::L0, F::L3, F::L6,
        F::U8, F::U7, F::U6, F::U5, F::U4, F::U3, F::U2, F::U1, F::U0,
    ]);

    pub const Y_CUBE: FaceletCube = FaceletCube([
        F::U6, F::U3, F::U0, F::U7, F::U4, F::U1, F::U8, F::U5, F::U2,
        F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
        F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
        F::D2, F::D5, F::D8, F::D1, F::D4, F::D7, F::D0, F::D3, F::D6,
        F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
        F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
    ]);

    pub const Z_CUBE: FaceletCube = FaceletCube([
        F::L6, F::L3, F::L0, F::L7, F::L4, F::L1, F::L8, F::L5, F::L2,
        F::U6, F::U3, F::U0, F::U7, F::U4, F::U1, F::U8, F::U5, F::U2,
        F::F6, F::F3, F::F0, F::F7, F::F4, F::F1, F::F8, F::F5, F::F2,
        F::R6, F::R3, F::R0, F::R7, F::R4, F::R1, F::R8, F::R5, F::R2,
        F::D6, F::D3, F::D0, F::D7, F::D4, F::D1, F::D8, F::D5, F::D2,
        F::B2, F::B5, F::B8, F::B1, F::B4, F::B7, F::B0, F::B3, F::B6,
    ]);

    pub const UW_CUBE: FaceletCube = FaceletCube([
        F::U6, F::U3, F::U0, F::U7, F::U4, F::U1, F::U8, F::U5, F::U2,
        F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::R6, F::R7, F::R8,
        F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::F6, F::F7, F::F8,
        F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::L6, F::L7, F::L8,
        F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::B6, F::B7, F::B8,
    ]);

    pub const RW_CUBE: FaceletCube = FaceletCube([
        F::U0, F::F1, F::F2, F::U3, F::F4, F::F5, F::U6, F::F7, F::F8,
        F::R6, F::R3, F::R0, F::R7, F::R4, F::R1, F::R8, F::R5, F::R2,
        F::F0, F::D1, F::D2, F::F3, F::D4, F::D5, F::F6, F::D7, F::D8,
        F::D0, F::B7, F::B6, F::D3, F::B4, F::B3, F::D6, F::B1, F::B0,
        F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
        F::U8, F::U7, F::B2, F::U5, F::U4, F::B5, F::U2, F::U1, F::B8,
    ]);

    pub const FW_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::U2, F::L7, F::L4, F::L1, F::L8, F::L5, F::L2,
        F::U6, F::U3, F::R2, F::U7, F::U4, F::R5, F::U8, F::U5, F::R8,
        F::F6, F::F3, F::F0, F::F7, F::F4, F::F1, F::F8, F::F5, F::F2,
        F::R6, F::R3, F::R0, F::R7, F::R4, F::R1, F::D6, F::D7, F::D8,
        F::L0, F::D3, F::D0, F::L3, F::D4, F::D1, F::L6, F::D5, F::D2,
        F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    ]);

    pub const DW_CUBE: FaceletCube = FaceletCube([
        F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
        F::R0, F::R1, F::R2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
        F::F0, F::F1, F::F2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
        F::D6, F::D3, F::D0, F::D7, F::D4, F::D1, F::D8, F::D5, F::D2,
        F::L0, F::L1, F::L2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
        F::B0, F::B1, F::B2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
    ]);

    pub const LW_CUBE: FaceletCube = FaceletCube([
        F::B8, F::B7, F::U2, F::B5, F::B4, F::U5, F::B2, F::B1, F::U8,
        F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
        F::U0, F::U1, F::F2, F::U3, F::U4, F::F5, F::U6, F::U7, F::F8,
        F::F0, F::F1, F::D2, F::F3, F::F4, F::D5, F::F6, F::F7, F::D8,
        F::L6, F::L3, F::L0, F::L7, F::L4, F::L1, F::L8, F::L5, F::L2,
        F::B0, F::D7, F::D6, F::B3, F::D4, F::D3, F::B6, F::D1, F::D0,
    ]);

    pub const BW_CUBE: FaceletCube = FaceletCube([
        F::R2, F::R5, F::R8, F::R1, F::R4, F::R7, F::U6, F::U7, F::U8,
        F::R0, F::D5, F::D8, F::R3, F::D4, F::D7, F::R6, F::D3, F::D6,
        F::F0, F::F1, F::F2, F::F3 ,F::F4, F::F5, F::F6, F::F7, F::F8,
        F::D0, F::D1, F::D2, F::L1, F::L4, F::L7, F::L0, F::L3, F::L6,
        F::U2, F::U5, F::L2, F::U1, F::U4, F::L5, F::U0, F::U3, F::L8,
        F::B6, F::B3, F::B0, F::B7, F::B4, F::B1, F::B8, F::B5, F::B2,
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{alg, moves::Inverse, sticker::Corner};
    use std::str::FromStr;

    #[test]
    fn test_primitive_moves() {
        let scramble = alg!("U R F D L B");
        let cube = FaceletCube::default().apply_alg(&scramble);
        let expected = "BBDBUFLLFURRURBDDLUFRUFRLLBFDRFDRUBBRLFULFBDDFUURBLDDL";

        assert_eq!(expected, cube.to_string());
    }

    #[test]
    fn test_scramble() {
        let scramble = alg!("D F2 U' B2 F2 U2 L2 D B2 D2 U' F2 U' F2 R' B R' D R2 D2 R' F' L R'");
        let cube = FaceletCube::default().apply_alg(&scramble);
        let expected = "FRDRULDFFRBLLRRFUURDDUFFLBLURDBDFLURUDBDLBUDFBURFBLBLB";

        assert_eq!(expected, cube.to_string());
    }

    #[test]
    fn test_slice_moves() {
        let scramble = alg!("M E S E' S' M'");
        let cube = FaceletCube::default().apply_alg(&scramble);
        let expected = "UUUUBUUUURRRRURRRRFFFFLFFFFDDDDFDDDDLLLLDLLLLBBBBRBBBB";

        assert_eq!(expected, cube.to_string());
    }

    #[test]
    fn test_rotations() {
        let scramble = alg!("x y z");
        let cube = FaceletCube::default().apply_alg(&scramble);
        let expected = "DDDDDDDDDFFFFFFFFFRRRRRRRRRUUUUUUUUUBBBBBBBBBLLLLLLLLL";

        assert_eq!(expected, cube.to_string());
    }

    #[test]
    fn test_wide_moves() {
        let scramble = alg!("u r f d l b");
        let cube = FaceletCube::default().apply_alg(&scramble);
        let expected = "BDDUDDLUFURRDLLDFLURRLFBLFBFRRUULUUBRFFRRDBBDFFUBBBDLL";

        assert_eq!(expected, cube.to_string());
    }

    #[test]
    fn test_solved_state() {
        let cube = FaceletCube::default();

        assert!(cube.is_solved());

        let scramble = alg!("x y2 z'");
        let cube = FaceletCube::default().apply_alg(&scramble);

        assert!(cube.is_solved());

        let scramble = alg!("R U R' U'");
        let cube = FaceletCube::default().apply_alg(&scramble);

        assert!(!cube.is_solved());
    }

    #[test]
    fn test_edge_cycle() {
        let cycle = Cycle::new(Edge::UF, Edge::UB, Edge::FL);
        let cube = FaceletCube::default().edge_cycle(cycle).unwrap();

        #[rustfmt::skip]
        let expecte = FaceletCube([
            F::U0, F::U7, F::U2, F::U3, F::U4, F::U5, F::U6, F::F3, F::U8,
            F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
            F::F0, F::L5, F::F2, F::U1, F::F4, F::F5, F::F6, F::F7, F::F8,
            F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
            F::L0, F::L1, F::L2, F::L3, F::L4, F::B1, F::L6, F::L7, F::L8,
            F::B0, F::F1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
        ]);

        assert_eq!(expecte, cube);

        let cube = cube.edge_cycle(cycle.inverse()).unwrap();

        assert_eq!(FaceletCube::default(), cube);
    }

    #[test]
    fn test_corner_cycle() {
        let cycle = Cycle::new(Corner::UFR, Corner::ULF, Corner::RFD);
        let cube = FaceletCube::default().corner_cycle(cycle).unwrap();

        #[rustfmt::skip]
        let expecte = FaceletCube([
            F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U8, F::U7, F::R6,
            F::D2, F::R1, F::R2, F::R3, F::R4, F::R5, F::U6, F::R7, F::R8,
            F::R0, F::F1, F::F8, F::F3, F::F4, F::F5, F::F6, F::F7, F::L2,
            F::D0, F::D1, F::F0, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
            F::L0, F::L1, F::F2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
            F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
        ]);

        assert_eq!(expecte, cube);

        let cube = cube.corner_cycle(cycle.inverse()).unwrap();

        assert_eq!(FaceletCube::default(), cube);
    }
}
