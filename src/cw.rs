mod mission;
mod poly;
mod vessel;

pub use poly::default_vessels;
use std::fmt::{self, Display};
use std::str::FromStr;
pub use vessel::{Vessel, VesselData};

/// Campaign regions
///
/// This enum describes those regions wherein a given vessel is a member of OP FORCE. For
/// simplicity, the Nato/Pact/Whatever suffix refers to the player's faction. Note that the
/// China Sea region does not have a suffix because the opposing force for all Chinese
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CampaignRegion {
    All,
    ChinaSeaNato,
    ChinaSeaPlan,
    NorwegianSeaBrit,
    NorwegianSeaNato,
    NorwegianSeaPact,
}

// FIXME: these types are probably wrong.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ShipType {
    Capital,
    Escort,
    Merchant,
    Submarine,
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
            "*" => Ok(CampaignRegion::All),
            "CHINA_SEA_NATO" => Ok(CampaignRegion::ChinaSeaNato),
            "CHINA_SEA_PLAN" => Ok(CampaignRegion::ChinaSeaPlan),
            "NORWEGIAN_SEA_BRIT" => Ok(CampaignRegion::NorwegianSeaBrit),
            "NORWEGIAN_SEA_NATO" => Ok(CampaignRegion::NorwegianSeaNato),
            "NORWEGIAN_SEA_PACT" => Ok(CampaignRegion::NorwegianSeaPact),
            _ => Err(InvalidData::CampaignRegion),
        }
    }
}

impl FromStr for ShipType {
    type Err = InvalidData;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CAPITAL" => Ok(ShipType::Capital),
            "ESCORT" => Ok(ShipType::Escort),
            "MERCHANT" => Ok(ShipType::Merchant),
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
