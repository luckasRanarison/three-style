use crate::facelet::Facelet;

pub const EDGE_FACELET_MAP: [[Facelet; 2]; 24] = [
    [Facelet::U1, Facelet::B1], // UB
    [Facelet::B1, Facelet::U1], // BU
    [Facelet::U5, Facelet::R1], // UR
    [Facelet::R1, Facelet::U5], // RU
    [Facelet::U7, Facelet::F1], // UF
    [Facelet::F1, Facelet::U7], // FU
    [Facelet::U3, Facelet::L1], // UL
    [Facelet::L1, Facelet::U3], // LU
    [Facelet::F3, Facelet::L5], // FL
    [Facelet::L5, Facelet::F3], // LF
    [Facelet::F5, Facelet::R3], // FR
    [Facelet::R3, Facelet::F5], // RF
    [Facelet::B3, Facelet::R5], // BR
    [Facelet::R5, Facelet::B3], // RB
    [Facelet::B5, Facelet::L3], // BL
    [Facelet::L3, Facelet::B5], // LB
    [Facelet::D1, Facelet::F7], // DF
    [Facelet::F7, Facelet::D1], // FD
    [Facelet::D5, Facelet::R7], // DR
    [Facelet::R7, Facelet::D5], // RD
    [Facelet::D7, Facelet::B7], // DB
    [Facelet::B7, Facelet::D7], // BD
    [Facelet::D3, Facelet::L7], // DL
    [Facelet::L7, Facelet::D3], // LD
];

pub const CORNER_FACELET_MAP: [[Facelet; 3]; 24] = [
    [Facelet::U0, Facelet::B2, Facelet::L0], // UBL
    [Facelet::B2, Facelet::L0, Facelet::U0], // BLU
    [Facelet::L0, Facelet::U0, Facelet::B2], // LUB
    [Facelet::U2, Facelet::R2, Facelet::B0], // URB
    [Facelet::R2, Facelet::B0, Facelet::U2], // RBU
    [Facelet::B0, Facelet::U2, Facelet::R2], // BUR
    [Facelet::U8, Facelet::F2, Facelet::R0], // UFR
    [Facelet::F2, Facelet::R0, Facelet::U8], // FRU
    [Facelet::R0, Facelet::U8, Facelet::F2], // RUF
    [Facelet::U6, Facelet::L2, Facelet::F0], // ULF
    [Facelet::L2, Facelet::F0, Facelet::U6], // LFU
    [Facelet::F0, Facelet::U6, Facelet::L2], // FUL
    [Facelet::D0, Facelet::F6, Facelet::L8], // DFL
    [Facelet::F6, Facelet::L8, Facelet::D0], // FLD
    [Facelet::L8, Facelet::D0, Facelet::F6], // LDF
    [Facelet::D2, Facelet::R6, Facelet::F8], // DRF
    [Facelet::R6, Facelet::F8, Facelet::D2], // RFD
    [Facelet::F8, Facelet::D2, Facelet::R6], // FDR
    [Facelet::D8, Facelet::B6, Facelet::R8], // DBR
    [Facelet::B6, Facelet::R8, Facelet::D8], // BRD
    [Facelet::R8, Facelet::D8, Facelet::B6], // RDB
    [Facelet::D6, Facelet::L6, Facelet::B8], // DLB
    [Facelet::L6, Facelet::B8, Facelet::D6], // LBD
    [Facelet::B8, Facelet::D6, Facelet::L6], // BDL
];
