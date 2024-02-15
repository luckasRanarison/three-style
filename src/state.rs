use crate::{
    facelet::*,
    moves::{Move, MoveKind},
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
const E_CUBE: Cube = Cube(E_STATE);
const S_CUBE: Cube = Cube(S_STATE);
const X_CUBE: Cube = Cube(X_STATE);
const Y_CUBE: Cube = Cube(Y_STATE);
const Z_CUBE: Cube = Cube(Z_STATE);

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

impl From<Move> for Cube {
    fn from(m: Move) -> Self {
        let count = m.direction as usize;
        let move_state = match m.kind {
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
            MoveKind::Fw => F_CUBE * S_CUBE,
            MoveKind::Lw => L_CUBE * M_CUBE,
            MoveKind::Dw => D_CUBE * E_CUBE,
            MoveKind::Uw => U_CUBE * E_CUBE.repeat(3),
            MoveKind::Rw => R_CUBE * M_CUBE.repeat(3),
            MoveKind::Bw => B_CUBE * S_CUBE.repeat(3),
        };

        move_state.repeat(count)
    }
}

impl Cube {
    pub fn apply_move(self, m: Move) -> Self {
        self * Cube::from(m)
    }

    pub fn apply_moves(self, moves: &[Move]) -> Self {
        moves.iter().fold(self, |acc, m| acc.apply_move(*m))
    }

    fn repeat(self, count: usize) -> Self {
        if count > 1 {
            vec![self.clone(); count - 1]
                .into_iter()
                .fold(self, |acc, m| acc * m)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{facelet::Facelet as F, moves::moves_from_str, state::Cube};

    fn cube_from_str(s: &str) -> Cube {
        let moves = moves_from_str(s).unwrap();
        Cube::default().apply_moves(&moves)
    }

    #[test]
    fn test_sexy_move() {
        let cube = cube_from_str("R U R' U' R U R' U' R U R' U' R U R' U' R U R' U' R U R' U'");
        assert_eq!(Cube::default(), cube);
        let cube = cube_from_str("L' U' L U L' U' L U L' U' L U L' U' L U L' U' L U L' U' L U");
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

    #[test]
    fn test_slice_moves() {
        let cube = cube_from_str("M E S");

        #[rustfmt::skip]
        let expected = Cube([
            F::U0, F::B7, F::U2, F::L7, F::D4, F::L1, F::U6, F::B1, F::U8,
            F::F0, F::U1, F::F2, F::L3, F::L4, F::L5, F::F6, F::U7, F::F8,
            F::R0, F::U3, F::R2, F::F3, F::B4, F::F5, F::R6, F::U5, F::R8,
            F::B0, F::D7, F::B2, F::R3, F::R4, F::R5, F::B6, F::D1, F::B8,
            F::L0, F::D3, F::L2, F::B3, F::F4, F::B5, F::L6, F::D5, F::L8,
            F::D0, F::F1, F::D2, F::R7, F::U4, F::R1, F::D6, F::F7, F::D8,
        ]);

        assert_eq!(expected, cube);
    }

    #[test]
    fn test_rotations() {
        let cube = cube_from_str("x y z");

        #[rustfmt::skip]
        let expecte = Cube([
            F::D6, F::D3, F::D0, F::D7, F::D4, F::D1, F::D8, F::D5, F::D2,
            F::R8, F::R7, F::R6, F::R5, F::R4, F::R3, F::R2, F::R1, F::R0,
            F::F8, F::F7, F::F6, F::F5, F::F4, F::F3, F::F2, F::F1, F::F0,
            F::L8, F::L7, F::L6, F::L5, F::L4, F::L3, F::L2, F::L1, F::L0,
            F::B8, F::B7, F::B6, F::B5, F::B4, F::B3, F::B2, F::B1, F::B0,
            F::U2, F::U5, F::U8, F::U1, F::U4, F::U7, F::U0, F::U3, F::U6,
        ]);

        assert_eq!(expecte, cube);
    }
}