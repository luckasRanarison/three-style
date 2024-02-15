use crate::{error::Error, facelet::Facelet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CornerOri {
    Normal,
    Clockwise,
    CounterClockwise,
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CornerKind {
    UBL, URB, UFR, ULF,
    DFL, DRF, DBR, DLB,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Corner {
    pub kind: CornerKind,
    pub orientation: CornerOri,
}

impl Corner {
    fn new(kind: CornerKind, orientation: CornerOri) -> Self {
        Self { kind, orientation }
    }
}

impl FromStr for Corner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UBL" => Ok(Self::new(CornerKind::UBL, CornerOri::Normal)),
            "BLU" => Ok(Self::new(CornerKind::UBL, CornerOri::Clockwise)),
            "LUB" => Ok(Self::new(CornerKind::UBL, CornerOri::CounterClockwise)),
            "URB" => Ok(Self::new(CornerKind::URB, CornerOri::Normal)),
            "RBU" => Ok(Self::new(CornerKind::URB, CornerOri::Clockwise)),
            "BUR" => Ok(Self::new(CornerKind::URB, CornerOri::CounterClockwise)),
            "UFR" => Ok(Self::new(CornerKind::UFR, CornerOri::Normal)),
            "FRU" => Ok(Self::new(CornerKind::UFR, CornerOri::Clockwise)),
            "RUF" => Ok(Self::new(CornerKind::UFR, CornerOri::CounterClockwise)),
            "ULF" => Ok(Self::new(CornerKind::ULF, CornerOri::Normal)),
            "LFU" => Ok(Self::new(CornerKind::ULF, CornerOri::Clockwise)),
            "FUL" => Ok(Self::new(CornerKind::ULF, CornerOri::CounterClockwise)),
            "DFL" => Ok(Self::new(CornerKind::DFL, CornerOri::Normal)),
            "FLD" => Ok(Self::new(CornerKind::DFL, CornerOri::Clockwise)),
            "LDF" => Ok(Self::new(CornerKind::DFL, CornerOri::CounterClockwise)),
            "DRF" => Ok(Self::new(CornerKind::DRF, CornerOri::Normal)),
            "RFD" => Ok(Self::new(CornerKind::DRF, CornerOri::Clockwise)),
            "FDR" => Ok(Self::new(CornerKind::DRF, CornerOri::CounterClockwise)),
            "DBR" => Ok(Self::new(CornerKind::DBR, CornerOri::Normal)),
            "BRD" => Ok(Self::new(CornerKind::DBR, CornerOri::Clockwise)),
            "RDB" => Ok(Self::new(CornerKind::DBR, CornerOri::CounterClockwise)),
            "DLB" => Ok(Self::new(CornerKind::DLB, CornerOri::Normal)),
            "LBD" => Ok(Self::new(CornerKind::DLB, CornerOri::Clockwise)),
            "BDL" => Ok(Self::new(CornerKind::DLB, CornerOri::CounterClockwise)),
            _ => Err(Error::InvalidCornerString(s.to_owned())),
        }
    }
}

const CORNER_FACELET_MAP: [[Facelet; 3]; 8] = [
    [Facelet::U0, Facelet::B2, Facelet::L0],
    [Facelet::U2, Facelet::R2, Facelet::B0],
    [Facelet::U8, Facelet::F2, Facelet::R0],
    [Facelet::U6, Facelet::L2, Facelet::F0],
    [Facelet::D0, Facelet::F6, Facelet::L8],
    [Facelet::D2, Facelet::R6, Facelet::F8],
    [Facelet::D8, Facelet::B6, Facelet::R8],
    [Facelet::D6, Facelet::L6, Facelet::B8],
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeOri {
    Normal,
    Fliped,
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeKind {
    UB, UR, UF, UL,
    FL, FR, BR, BL,
    DF, DR, DB, DL,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub kind: EdgeKind,
    pub orientation: EdgeOri,
}

impl Edge {
    fn new(kind: EdgeKind, orientation: EdgeOri) -> Self {
        Self { kind, orientation }
    }
}

impl FromStr for Edge {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UB" => Ok(Self::new(EdgeKind::UB, EdgeOri::Normal)),
            "BU" => Ok(Self::new(EdgeKind::UB, EdgeOri::Fliped)),
            "UR" => Ok(Self::new(EdgeKind::UR, EdgeOri::Normal)),
            "RU" => Ok(Self::new(EdgeKind::UR, EdgeOri::Fliped)),
            "UF" => Ok(Self::new(EdgeKind::UF, EdgeOri::Normal)),
            "FU" => Ok(Self::new(EdgeKind::UF, EdgeOri::Fliped)),
            "UL" => Ok(Self::new(EdgeKind::UL, EdgeOri::Normal)),
            "LU" => Ok(Self::new(EdgeKind::UL, EdgeOri::Fliped)),
            "FL" => Ok(Self::new(EdgeKind::FL, EdgeOri::Normal)),
            "LF" => Ok(Self::new(EdgeKind::FL, EdgeOri::Fliped)),
            "FR" => Ok(Self::new(EdgeKind::FR, EdgeOri::Normal)),
            "RF" => Ok(Self::new(EdgeKind::FR, EdgeOri::Fliped)),
            "BR" => Ok(Self::new(EdgeKind::BR, EdgeOri::Normal)),
            "RB" => Ok(Self::new(EdgeKind::BR, EdgeOri::Fliped)),
            "BL" => Ok(Self::new(EdgeKind::BL, EdgeOri::Normal)),
            "LB" => Ok(Self::new(EdgeKind::BL, EdgeOri::Fliped)),
            "DF" => Ok(Self::new(EdgeKind::DF, EdgeOri::Normal)),
            "FD" => Ok(Self::new(EdgeKind::DF, EdgeOri::Fliped)),
            "DR" => Ok(Self::new(EdgeKind::DR, EdgeOri::Normal)),
            "RD" => Ok(Self::new(EdgeKind::DR, EdgeOri::Fliped)),
            "DB" => Ok(Self::new(EdgeKind::DB, EdgeOri::Normal)),
            "BD" => Ok(Self::new(EdgeKind::DB, EdgeOri::Fliped)),
            "DL" => Ok(Self::new(EdgeKind::DL, EdgeOri::Normal)),
            "LD" => Ok(Self::new(EdgeKind::DL, EdgeOri::Fliped)),
            _ => Err(Error::InvalidEdgeString(s.to_owned())),
        }
    }
}

const EDGE_FACELET_MAP: [[Facelet; 2]; 12] = [
    [Facelet::U1, Facelet::B1],
    [Facelet::U5, Facelet::R1],
    [Facelet::U7, Facelet::F1],
    [Facelet::U3, Facelet::L1],
    [Facelet::F3, Facelet::L5],
    [Facelet::F6, Facelet::R3],
    [Facelet::B3, Facelet::R5],
    [Facelet::B5, Facelet::L3],
    [Facelet::D1, Facelet::F7],
    [Facelet::D5, Facelet::R7],
    [Facelet::D7, Facelet::B7],
    [Facelet::D3, Facelet::L7],
];
