#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Facelet {
    U0, U1, U2, U3, U4, U5, U6, U7, U8,
    F0, F1, F2, F3, F4, F5, F6, F7, F8,
    R0, R1, R2, R3, R4, R5, R6, R7, R8,
    B0, B1, B2, B3, B4, B5, B6, B7, B8,
    L0, L1, L2, L3, L4, L5, L6, L7, L8,
    D0, D1, D2, D3, D4, D5, D6, D7, D8,
}

pub type FaceState = [Facelet; 54];

use Facelet as F;

#[rustfmt::skip]
pub const SOLVED_STATE: FaceState = [
    F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
    F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
    F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
    F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
    F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
];

#[rustfmt::skip]
pub const U_STATE: FaceState = [
    F::U6, F::U3, F::U0, F::U7, F::U4, F::U1, F::U8, F::U5, F::U2,
    F::R0, F::R1, F::R2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
    F::B0, F::B1, F::B2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
    F::L0, F::L1, F::L2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    F::F0, F::F1, F::F2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
    F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
];

#[rustfmt::skip]
pub const F_STATE: FaceState = [
    F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::L8, F::L5, F::L2,
    F::F6, F::F3, F::F0, F::F7, F::F4, F::F1, F::F8, F::F5, F::F2,
    F::U6, F::R1, F::R2, F::U7, F::R4, F::R5, F::U8, F::R7, F::R8,
    F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::B6, F::B7, F::B8,
    F::L0, F::L1, F::D0, F::L3, F::L4, F::D1, F::L6, F::L7, F::D2,
    F::R6, F::R3, F::R0, F::D3, F::D4, F::D5, F::D6, F::D7, F::D8,
];

#[rustfmt::skip]
pub const R_STATE: FaceState = [
    F::U0, F::U1, F::F2, F::U3, F::U4, F::F5, F::U6, F::U7, F::F8,
    F::F0, F::F1, F::D2, F::F3, F::F4, F::D5, F::F6, F::F7, F::D8,
    F::R6, F::R3, F::R0, F::R7, F::R4, F::R1, F::R8, F::R5, F::R2,
    F::U8, F::B1, F::B2, F::U5, F::B4, F::B5, F::U2, F::B7, F::B8,
    F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
    F::D0, F::D1, F::B6, F::D3, F::D4, F::B3, F::D6, F::D7, F::B0,
];

#[rustfmt::skip]
pub const B_STATE: FaceState = [
    F::R2, F::R5, F::R8, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
    F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::F6, F::F7, F::F8,
    F::R0, F::R1, F::D8, F::R3, F::R4, F::D7, F::R6, F::R7, F::D6,
    F::B6, F::B3, F::B0, F::B7, F::B4, F::B1, F::B8, F::B5, F::B2,
    F::U2, F::L1, F::L2, F::U1, F::L4, F::L5, F::U0, F::L7, F::L8,
    F::D0, F::D1, F::D2, F::D3, F::D4, F::D5, F::L0, F::L3, F::L6,
];

#[rustfmt::skip]
pub const L_STATE: FaceState = [
    F::B8, F::U1, F::U2, F::B5, F::U4, F::U5, F::B2, F::U7, F::U8,
    F::U0, F::F1, F::F2, F::U3, F::F4, F::F5, F::U6, F::F7, F::F8,
    F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
    F::B0, F::B1, F::D6, F::B3, F::B4, F::D3, F::B6, F::B7, F::D0,
    F::L6, F::L3, F::L0, F::L7, F::L4, F::L1, F::L8, F::L5, F::L2,
    F::F0, F::D1, F::D2, F::F3, F::D4, F::D5, F::F6, F::D7, F::D8,
];

#[rustfmt::skip]
pub const D_STATE: FaceState = [
    F::U0, F::U1, F::U2, F::U3, F::U4, F::U5, F::U6, F::U7, F::U8,
    F::F0, F::F1, F::F2, F::F3, F::F4, F::F5, F::L6, F::L7, F::L8,
    F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::F6, F::F7, F::F8,
    F::B0, F::B1, F::B2, F::B3, F::B4, F::B5, F::R6, F::R7, F::R8,
    F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::B6, F::B7, F::B8,
    F::D6, F::D3, F::D0, F::D7, F::D4, F::D1, F::D8, F::D5, F::D2,
];

#[rustfmt::skip]
pub const M_STATE: FaceState = [
    F::U0, F::B7, F::U2, F::U3, F::B4, F::U5, F::U6, F::B1, F::U8,
    F::F0, F::U1, F::F2, F::F3, F::U4, F::F5, F::F6, F::U7, F::F8,
    F::R0, F::R1, F::R2, F::R3, F::R4, F::R5, F::R6, F::R7, F::R8,
    F::B0, F::D7, F::B2, F::B3, F::D4, F::B5, F::B6, F::D1, F::B8,
    F::L0, F::L1, F::L2, F::L3, F::L4, F::L5, F::L6, F::L7, F::L8,
    F::D0, F::F1, F::D2, F::D3, F::F4, F::D5, F::D6, F::F7, F::D8,
];
