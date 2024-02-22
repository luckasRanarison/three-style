mod constants;

use crate::{
    error::Error,
    facelet::{Facelet, FaceletTarget},
};
use constants::{CORNER_FACELET_MAP, EDGE_FACELET_MAP};
use std::{fmt, str::FromStr};

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Corner {
    UBL, BLU, LUB,
    URB, RBU, BUR,
    UFR, FRU, RUF,
    ULF, LFU, FUL,
    DFL, FLD, LDF,
    DRF, RFD, FDR,
    DBR, BRD, RDB,
    DLB, LBD, BDL,
}

impl FaceletTarget for Corner {
    fn to_facelets(&self) -> Vec<Facelet> {
        CORNER_FACELET_MAP[*self as usize].to_vec()
    }
}

impl FromStr for Corner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UBL" => Ok(Self::UBL),
            "BLU" => Ok(Self::BLU),
            "LUB" => Ok(Self::LUB),
            "URB" => Ok(Self::URB),
            "RBU" => Ok(Self::RBU),
            "BUR" => Ok(Self::BUR),
            "UFR" => Ok(Self::UFR),
            "FRU" => Ok(Self::FRU),
            "RUF" => Ok(Self::RUF),
            "ULF" => Ok(Self::ULF),
            "LFU" => Ok(Self::LFU),
            "FUL" => Ok(Self::FUL),
            "DFL" => Ok(Self::DFL),
            "FLD" => Ok(Self::FLD),
            "LDF" => Ok(Self::LDF),
            "DRF" => Ok(Self::DRF),
            "RFD" => Ok(Self::RFD),
            "FDR" => Ok(Self::FDR),
            "DBR" => Ok(Self::DBR),
            "BRD" => Ok(Self::BRD),
            "RDB" => Ok(Self::RDB),
            "DLB" => Ok(Self::DLB),
            "LBD" => Ok(Self::LBD),
            "BDL" => Ok(Self::BDL),
            _ => Err(Error::InvalidCornerString(s.to_owned())),
        }
    }
}

impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Edge {
    UB, BU,
    UR, RU,
    UF, FU,
    UL, LU,
    FL, LF,
    FR, RF,
    BR, RB,
    BL, LB,
    DF, FD,
    DR, RD,
    DB, BD,
    DL, LD,
}

impl FaceletTarget for Edge {
    fn to_facelets(&self) -> Vec<Facelet> {
        EDGE_FACELET_MAP[*self as usize].to_vec()
    }
}

impl FromStr for Edge {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UB" => Ok(Self::UB),
            "BU" => Ok(Self::BU),
            "UR" => Ok(Self::UR),
            "RU" => Ok(Self::RU),
            "UF" => Ok(Self::UF),
            "FU" => Ok(Self::FU),
            "UL" => Ok(Self::UL),
            "LU" => Ok(Self::LU),
            "FL" => Ok(Self::FL),
            "LF" => Ok(Self::LF),
            "FR" => Ok(Self::FR),
            "RF" => Ok(Self::RF),
            "BR" => Ok(Self::BR),
            "RB" => Ok(Self::RB),
            "BL" => Ok(Self::BL),
            "LB" => Ok(Self::LB),
            "DF" => Ok(Self::DF),
            "FD" => Ok(Self::FD),
            "DR" => Ok(Self::DR),
            "RD" => Ok(Self::RD),
            "DB" => Ok(Self::DB),
            "BD" => Ok(Self::BD),
            "DL" => Ok(Self::DL),
            "LD" => Ok(Self::LD),
            _ => Err(Error::InvalidEdgeString(s.to_owned())),
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
