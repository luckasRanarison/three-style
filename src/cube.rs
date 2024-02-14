use crate::{
    facelet::{
        FaceState, B_STATE, D_STATE, F_STATE, L_STATE, M_STATE, R_STATE, SOLVED_STATE, U_STATE,
    },
    moves::{Move, MoveType},
};
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Cube(FaceState);

const U_CUBE: Cube = Cube(U_STATE);
const F_CUBE: Cube = Cube(F_STATE);
const R_CUBE: Cube = Cube(R_STATE);
const B_CUBE: Cube = Cube(B_STATE);
const L_CUBE: Cube = Cube(L_STATE);
const D_CUBE: Cube = Cube(D_STATE);
const M_CUBE: Cube = Cube(M_STATE);

impl Default for Cube {
    fn default() -> Self {
        Self(SOLVED_STATE)
    }
}

impl Mul<Cube> for Cube {
    type Output = Cube;

    fn mul(self, rhs: Cube) -> Self::Output {
        let mut res = Cube::default();

        for i in 0..54 {
            res.0[i] = self.0[rhs.0[i] as usize];
        }

        res
    }
}

impl Cube {
    pub fn apply_move(self, m: Move) -> Self {
        match m.get_type() {
            MoveType::Normal => self.apply_single_move(m),
            MoveType::Double => self.apply_single_move(m).apply_single_move(m),
            MoveType::Prime => self
                .apply_single_move(m)
                .apply_single_move(m)
                .apply_single_move(m),
        }
    }

    pub fn apply_moves(self, moves: &[Move]) -> Self {
        moves.iter().fold(self, |acc, m| acc.apply_move(*m))
    }

    fn apply_single_move(self, m: Move) -> Self {
        match m {
            Move::U(_) => self * U_CUBE,
            Move::F(_) => self * F_CUBE,
            Move::R(_) => self * R_CUBE,
            Move::B(_) => self * B_CUBE,
            Move::L(_) => self * L_CUBE,
            Move::D(_) => self * D_CUBE,
            Move::M(_) => self * M_CUBE,
            Move::S(_) => todo!(),
            Move::E(_) => todo!(),
            Move::X(_) => todo!(),
            Move::Y(_) => todo!(),
            Move::Z(_) => todo!(),
            Move::Uw(_) => todo!(),
            Move::Fw(_) => todo!(),
            Move::Rw(_) => todo!(),
            Move::Bw(_) => todo!(),
            Move::Lw(_) => todo!(),
            Move::Dw(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cube::Cube, facelet::Facelet as F, moves::moves_from_str};

    fn cube_from_str(s: &str) -> Cube {
        let moves = moves_from_str(s).unwrap();
        Cube::default().apply_moves(&moves)
    }

    #[test]
    fn test_sexy_move() {
        let cube = cube_from_str("R U R' U' R U R' U' R U R' U' R U R' U' R U R' U' R U R' U'");
        assert_eq!(Cube::default(), cube);
    }

    #[test]
    fn test_primitive_moves() {
        let cube = cube_from_str("R U R' F' R U R' U' R' F R2 U' R' U'");

        #[rustfmt::skip]
        let expected = Cube([
            F::U0, F::U1, F::U8, F::U3, F::U4, F::U7, F::U6, F::U5, F::U2,
            F::F0, F::R1, F::R2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
            F::B0, F::F1, F::F2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
            F::R0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
            F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
            F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        ]);

        assert_eq!(expected, cube);
    }
}
