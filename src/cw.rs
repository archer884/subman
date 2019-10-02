mod vessel;

use std::fmt::{self, Display};
use std::str::FromStr;
pub use vessel::{Vessel, VesselData};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CampaignRegion {
    ChinaSea,
    NorwegianSea,
}

// FIXME: these types are probably wrong.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShipType {
    Merchant,
    Submarine,
    Surface,
}

#[derive(Debug)]
pub enum InvalidData {
    CampaignRegion,
    ShipType,
    Numeric,
}

impl FromStr for CampaignRegion {
    type Err = InvalidData;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHINASEA" => Ok(CampaignRegion::ChinaSea),
            "NORWEGIANSEA" => Ok(CampaignRegion::NorwegianSea),
            _ => Err(InvalidData::CampaignRegion),
        }
    }
}

impl FromStr for ShipType {
    type Err = InvalidData;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUBMARINE" => Ok(ShipType::Submarine),
            _ => Err(InvalidData::ShipType),
        }
    }
}

impl Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShipType::Submarine => f.write_str("Submarine"),
            _ => unreachable!(),
        }
    }
}

impl Display for InvalidData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidData::CampaignRegion => f.write_str("Invalid campaign region"),
            InvalidData::Numeric => f.write_str("Invalid numeric value"),
            InvalidData::ShipType => f.write_str("Invalid ship type"),
        }
    }
}
