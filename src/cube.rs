use crate::moves::{Move, MoveType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    U,
    F,
    R,
    B,
    L,
    D,
}

#[derive(Debug, PartialEq)]
pub struct Cube {
    u: [Color; 9],
    f: [Color; 9],
    r: [Color; 9],
    b: [Color; 9],
    l: [Color; 9],
    d: [Color; 9],
}

impl Default for Cube {
    #[rustfmt::skip]
    fn default() -> Self {
        Cube {
            u: [ Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U ],
            f: [ Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F, Color::F ],
            r: [ Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R ],
            b: [ Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B ],
            l: [ Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L ],
            d: [ Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D ],
        }
    }
}

#[allow(non_snake_case)]
impl Cube {
    pub fn apply_move(&mut self, m: Move) {
        match m.get_type() {
            MoveType::Prime => self.apply_move(m.derivate(MoveType::Double)),
            MoveType::Double => self.apply_move(m.derivate(MoveType::Normal)),
            MoveType::Normal => {}
        }

        match m {
            Move::U(_) => self.apply_U(),
            Move::F(_) => self.apply_F(),
            Move::R(_) => self.apply_R(),
            Move::B(_) => self.apply_B(),
            Move::L(_) => self.apply_L(),
            Move::D(_) => self.apply_D(),
            Move::M(_) => self.apply_M(),
            Move::E(_) => self.apply_E(),
            Move::S(_) => self.apply_S(),
            Move::u(_) => self.apply_u(),
            Move::f(_) => self.apply_f(),
            Move::r(_) => self.apply_r(),
            Move::b(_) => self.apply_b(),
            Move::l(_) => self.apply_l(),
            Move::d(_) => self.apply_d(),
            Move::x(_) => self.apply_x(),
            Move::y(_) => self.apply_y(),
            Move::z(_) => self.apply_z(),
        }
    }

    pub fn apply_moves(&mut self, moves: &[Move]) {
        for m in moves {
            self.apply_move(*m);
        }
    }

    fn apply_U(&mut self) {
        let tmp = self.f[0..3].to_vec();
        self.f[0..3].copy_from_slice(&self.r[0..3]);
        self.r[0..3].copy_from_slice(&self.b[0..3]);
        self.b[0..3].copy_from_slice(&self.l[0..3]);
        self.l[0..3].copy_from_slice(&tmp);
        rotate_face(&mut self.u, 1);
    }

    fn apply_F(&mut self) {
        let tmp = self.u[6..9].to_vec();
        self.u[6] = self.l[8];
        self.u[7] = self.l[5];
        self.u[8] = self.l[2];
        self.l[2] = self.d[0];
        self.l[5] = self.d[1];
        self.l[8] = self.d[2];
        self.d[0] = self.r[6];
        self.d[1] = self.r[3];
        self.d[2] = self.r[0];
        self.r[0] = tmp[0];
        self.r[3] = tmp[1];
        self.r[6] = tmp[2];
        rotate_face(&mut self.f, 1);
    }

    fn apply_R(&mut self) {
        let tmp = vec![self.u[2], self.u[5], self.u[8]];
        self.u[2] = self.f[2];
        self.u[5] = self.f[5];
        self.u[8] = self.f[8];
        self.f[2] = self.d[2];
        self.f[5] = self.d[5];
        self.f[8] = self.d[8];
        self.d[2] = self.b[6];
        self.d[5] = self.b[3];
        self.d[8] = self.b[0];
        self.b[0] = tmp[2];
        self.b[3] = tmp[1];
        self.b[6] = tmp[0];
        rotate_face(&mut self.r, 1);
    }

    fn apply_B(&mut self) {
        let tmp = self.u[0..3].to_vec();
        self.u[0] = self.r[2];
        self.u[1] = self.r[5];
        self.u[2] = self.r[8];
        self.r[2] = self.d[8];
        self.r[5] = self.d[7];
        self.r[8] = self.d[6];
        self.d[6] = self.l[0];
        self.d[7] = self.l[3];
        self.d[8] = self.l[6];
        self.l[0] = tmp[2];
        self.l[3] = tmp[1];
        self.l[6] = tmp[0];
        rotate_face(&mut self.b, 1);
    }

    fn apply_L(&mut self) {
        let tmp = vec![self.u[0], self.u[3], self.u[6]];
        self.u[0] = self.b[8];
        self.u[3] = self.b[5];
        self.u[6] = self.b[2];
        self.b[2] = self.d[6];
        self.b[5] = self.d[3];
        self.b[8] = self.d[0];
        self.d[0] = self.f[0];
        self.d[3] = self.f[3];
        self.d[6] = self.f[6];
        self.f[0] = tmp[0];
        self.f[3] = tmp[1];
        self.f[6] = tmp[2];
        rotate_face(&mut self.l, 1);
    }

    fn apply_D(&mut self) {
        let tmp = self.f[6..9].to_vec();
        self.f[6..9].copy_from_slice(&self.l[6..9]);
        self.l[6..9].copy_from_slice(&self.b[6..9]);
        self.b[6..9].copy_from_slice(&self.r[6..9]);
        self.r[6..9].copy_from_slice(&tmp);
        rotate_face(&mut self.d, 1);
    }

    fn apply_x(&mut self) {
        let tmp = self.f;
        self.f = self.d;
        rotate_face(&mut self.b, 2);
        self.d = self.b;
        rotate_face(&mut self.u, 2);
        self.b = self.u;
        self.u = tmp;
        rotate_face(&mut self.r, 1);
        rotate_face(&mut self.l, 3);
    }

    fn apply_y(&mut self) {
        let tmp = self.f;
        self.f = self.r;
        self.r = self.b;
        self.b = self.l;
        self.l = tmp;
        rotate_face(&mut self.u, 1);
        rotate_face(&mut self.d, 3);
    }

    fn apply_z(&mut self) {
        let tmp = self.u;
        rotate_face(&mut self.l, 1);
        self.u = self.l;
        rotate_face(&mut self.d, 1);
        self.l = self.d;
        rotate_face(&mut self.r, 1);
        self.d = self.r;
        self.r = tmp;
        rotate_face(&mut self.r, 1);
        rotate_face(&mut self.f, 1);
        rotate_face(&mut self.b, 3);
    }

    fn apply_M(&mut self) {
        let tmp = vec![self.u[1], self.u[4], self.u[7]];
        self.u[1] = self.b[7];
        self.u[4] = self.b[4];
        self.u[7] = self.b[1];
        self.b[1] = self.d[7];
        self.b[4] = self.d[4];
        self.b[7] = self.d[1];
        self.d[1] = self.f[1];
        self.d[4] = self.f[4];
        self.d[7] = self.f[7];
        self.f[1] = tmp[0];
        self.f[4] = tmp[1];
        self.f[7] = tmp[2];
    }

    fn apply_E(&mut self) {
        let tmp = self.f[3..6].to_vec();
        self.f[3..6].copy_from_slice(&self.l[3..6]);
        self.l[3..6].copy_from_slice(&self.b[3..6]);
        self.b[3..6].copy_from_slice(&self.r[3..6]);
        self.r[3..6].copy_from_slice(&tmp);
    }

    fn apply_S(&mut self) {
        let tmp = self.u[3..6].to_vec();
        self.u[3] = self.l[7];
        self.u[4] = self.l[4];
        self.u[5] = self.l[1];
        self.l[1] = self.d[3];
        self.l[4] = self.d[4];
        self.l[7] = self.d[5];
        self.d[3] = self.r[7];
        self.d[4] = self.r[4];
        self.d[5] = self.r[1];
        self.r[1] = tmp[0];
        self.r[4] = tmp[1];
        self.r[7] = tmp[2];
    }

    fn apply_u(&mut self) {
        self.apply_U();
        self.apply_move(Move::E(MoveType::Prime));
    }

    fn apply_f(&mut self) {
        self.apply_F();
        self.apply_S();
    }

    fn apply_r(&mut self) {
        self.apply_R();
        self.apply_move(Move::M(MoveType::Prime));
    }

    fn apply_b(&mut self) {
        self.apply_B();
        self.apply_move(Move::S(MoveType::Prime));
    }

    fn apply_l(&mut self) {
        self.apply_L();
        self.apply_M();
    }

    fn apply_d(&mut self) {
        self.apply_D();
        self.apply_E();
    }
}

fn rotate_face(face: &mut [Color; 9], count: u8) {
    for _ in 0..count {
        let tmp = face[0..3].to_vec();
        face[2] = face[0];
        face[1] = face[3];
        face[0] = face[6];
        face[3] = face[7];
        face[6] = face[8];
        face[7] = face[5];
        face[8] = tmp[2];
        face[5] = tmp[1];
    }
}

#[cfg(test)]
mod tests {
    use super::Cube;
    use crate::{cube::Color, moves::moves_from_str};

    fn cube_from_moves(s: &str) -> Cube {
        let moves = moves_from_str(s).unwrap();
        let mut cube = Cube::default();
        cube.apply_moves(&moves);
        cube
    }

    #[test]
    fn test_sexy_move() {
        let cube = cube_from_moves("R U R' U' R U R' U' R U R' U' R U R' U' R U R' U' R U R' U'");

        assert_eq!(Cube::default(), cube);
    }

    #[test]
    fn test_primitive_moves() {
        let cube = cube_from_moves("F' R2 D2 B F D2 B L2 F2 U2 R2 L' D2 L' F R' D' B2 U2 F");

        #[rustfmt::skip]
        let expected = Cube {
            u: [ Color::F, Color::L, Color::U, Color::F, Color::U, Color::U, Color::U, Color::U, Color::L ],
            f: [ Color::B, Color::B, Color::F, Color::L, Color::F, Color::R, Color::D, Color::U, Color::L ],
            r: [ Color::U, Color::L, Color::B, Color::D, Color::R, Color::R, Color::D, Color::F, Color::R ],
            b: [ Color::L, Color::D, Color::L, Color::F, Color::B, Color::B, Color::F, Color::B, Color::F ],
            l: [ Color::D, Color::D, Color::R, Color::D, Color::L, Color::F, Color::R, Color::B, Color::B ],
            d: [ Color::R, Color::R, Color::B, Color::R, Color::D, Color::U, Color::D, Color::L, Color::U ],
        };

        assert_eq!(expected, cube);
    }

    #[test]
    fn test_rotations() {
        let cube = cube_from_moves("R U R' U' x y z");

        #[rustfmt::skip]
        let expected = Cube {
            u: [ Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::D, Color::R ],
            f: [ Color::R, Color::R, Color::U, Color::R, Color::R, Color::B, Color::U, Color::R, Color::R ],
            r: [ Color::F, Color::F, Color::F, Color::U, Color::F, Color::F, Color::D, Color::F, Color::F ],
            b: [ Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::L, Color::B ],
            l: [ Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::R, Color::R, Color::B ],
            d: [ Color::L, Color::F, Color::F, Color::U, Color::U, Color::U, Color::U, Color::U, Color::U ],
        };

        assert_eq!(expected, cube);
    }

    #[test]
    fn test_slice_moves() {
        let cube = cube_from_moves("M E S");

        #[rustfmt::skip]
        let expected = Cube {
            u: [ Color::U, Color::B, Color::U, Color::L, Color::D, Color::L, Color::U, Color::B, Color::U ],
            f: [ Color::F, Color::U, Color::F, Color::L, Color::L, Color::L, Color::F, Color::U, Color::F ],
            r: [ Color::R, Color::U, Color::R, Color::F, Color::B, Color::F, Color::R, Color::U, Color::R ],
            b: [ Color::B, Color::D, Color::B, Color::R, Color::R, Color::R, Color::B, Color::D, Color::B ],
            l: [ Color::L, Color::D, Color::L, Color::B, Color::F, Color::B, Color::L, Color::D, Color::L ],
            d: [ Color::D, Color::F, Color::D, Color::R, Color::U, Color::R, Color::D, Color::F, Color::D ],
        };

        assert_eq!(expected, cube);
    }

    #[test]
    fn test_wide_moves() {
        let cube = cube_from_moves("r u l f d b");

        #[rustfmt::skip]
        let expected = Cube {
            u: [ Color::B, Color::U, Color::R, Color::L, Color::F, Color::R, Color::D, Color::D, Color::F ],
            f: [ Color::F, Color::F, Color::U, Color::L, Color::R, Color::R, Color::L, Color::B, Color::B ],
            r: [ Color::L, Color::U, Color::U, Color::F, Color::U, Color::U, Color::D, Color::D, Color::B ],
            b: [ Color::F, Color::L, Color::L, Color::F, Color::L, Color::B, Color::R, Color::B, Color::D ],
            l: [ Color::U, Color::F, Color::R, Color::U, Color::D, Color::D, Color::B, Color::D, Color::D ],
            d: [ Color::F, Color::R, Color::R, Color::R, Color::B, Color::B, Color::L, Color::L, Color::U ],
        };

        assert_eq!(expected, cube);
    }
}
