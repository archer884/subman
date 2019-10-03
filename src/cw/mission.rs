use serde::Deserialize;

// FIXME: pretty sure that player vessels will not be required on this struct.

#[derive(Clone, Debug)]
struct MissionProfile {
    world_objects_data: String,
    vessels_and_traffic: String,
    date: u16,
    enemy_ship_classes: Vec<String>,
    player_vessels: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MissionProfileData {
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
            date: date.parse()?,
            enemy_ship_classes: extract_vessel_keys(enemy_ship_classes),
            player_vessels: extract_vessel_keys(player_vessels),
        })
    }
}

fn extract_vessel_keys(s: impl AsRef<str>) -> Vec<String> {
    s.as_ref()
        .split(|u: char| u == ',' || u == '|')
        .map(ToOwned::to_owned)
        .collect()
}
