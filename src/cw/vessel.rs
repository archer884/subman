use crate::cw::{CampaignRegion, InvalidData, ShipType};
use hashbrown::HashSet;
use serde::Deserialize;

pub struct Vessel {
    pub key: String,
    pub year: u16,
    pub ship_type: ShipType,
    pub op_force_regions: HashSet<CampaignRegion>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VesselData {
    ship_type: String,
    year: String,
    op_force_regions: String,
}

impl VesselData {
    pub fn with_key(self, key: impl Into<String>) -> crate::Result<Vessel> {
        let VesselData {
            ship_type,
            year,
            op_force_regions,
        } = self;

        Ok(Vessel {
            key: key.into(),
            ship_type: ship_type.parse()?,
            year: year.parse()?,
            op_force_regions: deserialize_regions(&op_force_regions)?,
        })
    }
}

fn deserialize_regions(s: &str) -> Result<HashSet<CampaignRegion>, InvalidData> {
    s.split(',').map(|s| s.parse::<CampaignRegion>()).collect()
}
