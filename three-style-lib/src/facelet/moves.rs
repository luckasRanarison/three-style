use super::{types::DEFAULT_STATE, Facelet, FaceletCube, FaceletState};
use crate::moves::Move;
use std::ops::Index;

#[derive(Debug, PartialEq, Clone)]
pub struct FaceletPermutation(FaceletState);

impl Index<Facelet> for FaceletPermutation {
    type Output = Facelet;

    fn index(&self, index: Facelet) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl Default for FaceletPermutation {
    fn default() -> Self {
        Self(DEFAULT_STATE)
    }
}

impl From<Move> for FaceletPermutation {
    fn from(value: Move) -> Self {
        let cube = FaceletCube::from(value);
        let default = FaceletPermutation::default();
        let mut res = FaceletPermutation::default();

        for i in 0..54 {
            res.0[cube[i] as usize] = default.0[i];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::FaceletPermutation;
    use crate::{
        facelet::Facelet as F,
        moves::{Move, MoveCount, MoveKind},
    };

    #[test]
    fn test_move_permutation() {
        let m = Move::new(MoveKind::U, MoveCount::Simple);

        #[rustfmt::skip]
        let expected = FaceletPermutation([
            F::U2, F::U5, F::U8, F::U1, F::U4, F::U7, F::U0, F::U3, F::U6,
            F::F0, F::F1, F::F2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
            F::L0, F::L1, F::L2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
            F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
            F::B0, F::B1, F::B2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
            F::R0, F::R1, F::R2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
        ]);

        assert_eq!(FaceletPermutation::from(m), expected);
    }
}
