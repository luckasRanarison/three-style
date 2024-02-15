use crate::{
    error::Error,
    facelet::*,
    moves::{Move, MoveKind},
    sticker::{CornerSticker, EdgeSticker},
};
use std::{collections::HashSet, ops::Mul};

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

trait ThreeCycle: Sized {
    fn edge_cycle(
        self,
        first: EdgeSticker,
        second: EdgeSticker,
        third: EdgeSticker,
    ) -> Result<Self, Error>;
    fn corner_cycle(
        self,
        first: CornerSticker,
        second: CornerSticker,
        third: CornerSticker,
    ) -> Result<Self, Error>;
}

impl ThreeCycle for Cube {
    fn edge_cycle(
        self,
        first: EdgeSticker,
        second: EdgeSticker,
        third: EdgeSticker,
    ) -> Result<Self, Error> {
        let mut res = self.clone();
        let first_facelets = first.into_facelet();
        let second_facelets = second.into_facelet();
        let third_facelets = third.into_facelet();
        let count = first_facelets
            .iter()
            .chain(second_facelets.iter())
            .chain(third_facelets.iter())
            .collect::<HashSet<_>>()
            .len();

        if count == 6 {
            res.0[first_facelets[0] as usize] = self.0[third_facelets[0] as usize];
            res.0[first_facelets[1] as usize] = self.0[third_facelets[1] as usize];
            res.0[second_facelets[0] as usize] = self.0[first_facelets[0] as usize];
            res.0[second_facelets[1] as usize] = self.0[first_facelets[1] as usize];
            res.0[third_facelets[0] as usize] = self.0[second_facelets[0] as usize];
            res.0[third_facelets[1] as usize] = self.0[second_facelets[1] as usize];

            Ok(res)
        } else {
            Err(Error::InvalidEdgeCycle(first, second, third))
        }
    }

    fn corner_cycle(
        self,
        first: CornerSticker,
        second: CornerSticker,
        third: CornerSticker,
    ) -> Result<Self, Error> {
        let mut res = self.clone();
        let first_facelets = first.into_facelet();
        let second_facelets = second.into_facelet();
        let third_facelets = third.into_facelet();
        let count = first_facelets
            .iter()
            .chain(second_facelets.iter())
            .chain(third_facelets.iter())
            .collect::<HashSet<_>>()
            .len();

        if count == 9 {
            res.0[first_facelets[0] as usize] = self.0[third_facelets[0] as usize];
            res.0[first_facelets[1] as usize] = self.0[third_facelets[1] as usize];
            res.0[first_facelets[2] as usize] = self.0[third_facelets[2] as usize];
            res.0[second_facelets[0] as usize] = self.0[first_facelets[0] as usize];
            res.0[second_facelets[1] as usize] = self.0[first_facelets[1] as usize];
            res.0[second_facelets[2] as usize] = self.0[first_facelets[2] as usize];
            res.0[third_facelets[0] as usize] = self.0[second_facelets[0] as usize];
            res.0[third_facelets[1] as usize] = self.0[second_facelets[1] as usize];
            res.0[third_facelets[2] as usize] = self.0[second_facelets[2] as usize];

            Ok(res)
        } else {
            Err(Error::InvalidCornerCycle(first, second, third))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreeCycle;
    use crate::{
        facelet::Facelet as F,
        moves::moves_from_str,
        state::Cube,
        sticker::{CornerSticker, EdgeSticker},
    };

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

    #[test]
    fn test_edge_cycle() {
        let cube = Cube::default()
            .edge_cycle(EdgeSticker::UF, EdgeSticker::UB, EdgeSticker::FL)
            .unwrap();

        #[rustfmt::skip]
        let expecte = Cube([
            F::U0, F::U7, F::U2, F::U3, F::U4, F::U5, F::U6, F::F3, F::U8,
            F::F0, F::L5, F::F2, F::U1, F::F4, F::F5, F::F6, F::F7, F::F8,
            F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
            F::B0, F::F1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
            F::L0, F::L1, F::L2, F::L3, F::L4, F::B1, F::L6, F::L7, F::L8,
            F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        ]);

        assert_eq!(expecte, cube);

        let cube = cube
            .edge_cycle(EdgeSticker::UF, EdgeSticker::FL, EdgeSticker::UB)
            .unwrap();

        assert_eq!(Cube::default(), cube);
    }

    #[test]
    fn test_corner_cycle() {
        let cube = Cube::default()
            .corner_cycle(CornerSticker::UFR, CornerSticker::ULF, CornerSticker::RFD)
            .unwrap();

        #[rustfmt::skip]
        let expecte = Cube([
            F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U8, F::U7, F::R6,
            F::R0, F::F1, F::F8, F::F3, F::F4, F::F5, F::F6, F::F7, F::L2,
            F::D2, F::R1, F::R2, F::R3, F::R4, F::R5, F::U6, F::R7, F::R8,
            F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
            F::L0, F::L1, F::F2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
            F::D0, F::D1, F::F0, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
        ]);

        assert_eq!(expecte, cube);

        let cube = cube
            .corner_cycle(CornerSticker::UFR, CornerSticker::RFD, CornerSticker::ULF)
            .unwrap();

        assert_eq!(Cube::default(), cube);
    }
}
