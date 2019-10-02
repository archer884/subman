use crate::cw::{CampaignRegion, InvalidData, ShipType};
use hashbrown::HashSet;
use serde::Deserialize;

pub struct Vessel {
    pub key: String,
    pub year: u16,
    pub ship_type: ShipType,
    pub player_fleet_regions: HashSet<CampaignRegion>,
    pub enemy_fleet_regions: HashSet<CampaignRegion>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VesselData {
    ship_type: String,
    year: String,
    player_fleet_regions: String,
    enemy_fleet_regions: String,
}

impl VesselData {
    pub fn with_key(self, key: impl Into<String>) -> crate::Result<Vessel> {
        let VesselData {
            ship_type,
            year,
            player_fleet_regions,
            enemy_fleet_regions,
        } = self;

        Ok(Vessel {
            key: key.into(),
            ship_type: ship_type.parse()?,
            year: year.parse()?,
            player_fleet_regions: deserialize_fleet_regions(&player_fleet_regions)?,
            enemy_fleet_regions: deserialize_fleet_regions(&enemy_fleet_regions)?,
        })
    }
}

fn deserialize_fleet_regions(s: &str) -> Result<HashSet<CampaignRegion>, InvalidData> {
    s.split(',').map(|s| s.parse::<CampaignRegion>()).collect()
}
