use crate::error::Error;
use serde::Deserialize;

// FIXME: pretty sure that player vessels will not be required on this struct.

#[derive(Clone, Debug)]
pub struct MissionProfile {
    world_objects_data: String,
    vessels_and_traffic: String,
    date: u16,
    enemy_ship_classes: Vec<String>,
    player_vessels: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionProfileData {
    world_objects_data: String,
    vessels_and_traffic: String,
    date: String,
    enemy_ship_classes: String,
    player_vessels: String,
}

impl MissionProfileData {
    pub fn into_profile(self) -> crate::Result<MissionProfile> {
        let MissionProfileData {
            world_objects_data,
            vessels_and_traffic,
            date,
            enemy_ship_classes,
            player_vessels,
        } = self;

        Ok(MissionProfile {
            world_objects_data,
            vessels_and_traffic,
            date: extract_date(&date)?,
            enemy_ship_classes: extract_vessel_keys(&enemy_ship_classes),
            player_vessels: extract_vessel_keys(&player_vessels),
        })
    }
}

fn extract_date(s: &str) -> crate::Result<u16> {
    let year = s
        .rsplit(' ')
        .next()
        .ok_or(Error::Data(crate::cw::InvalidData::Numeric))?;
    Ok(year.parse()?)
}

fn extract_vessel_keys(s: &str) -> Vec<String> {
    s.split(|u: char| u == ',' || u == '|')
        .map(ToOwned::to_owned)
        .collect()
}
