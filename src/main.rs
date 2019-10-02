mod cw;
mod error;
mod ini;

use cw::{Vessel, VesselData};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    /// Path of the game's default data directory
    default_path: String,

    /// Path of the mod's override directory
    override_path: String,
}

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct IniMissionProfile {
    use_terrain: String,
    map_coordinates: String,
    map_navigation_data: String,
    map_elevation_data: String,
    world_objects_data: String,
    vessels_and_traffic: String,
    date: String,
    hemisphere: String,
    time: String,
    use_preset_environment: String,
    weather: String,
    sea_state: String,
    duct_strength: String,
    layer_strength: String,
    number_of_enemy_units: String,

    // Renamed explicitly because BRITS CANNOT SPELL.
    #[serde(rename = "CombatBehaviour")]
    combat_behavior: String,

    enemy_ship_classes: String,
    formation_cruise_speed: String,
    number_of_helicopters: String,
    helicopter_type: String,
    number_of_aircraft: String,
    aircraft_type: String,
    player_vessels: String,
}

fn main() -> Result<()> {
    let Opt {
        default_path,
        override_path,
    } = Opt::from_args();

    let default_vessels = default_path + "/vessels";
    let override_vessels = override_path + "/vessels";

    for vessel in read_vessels(default_vessels) {
        println!("rel {} {}", vessel.ship_type, vessel.key);
    }

    for vessel in read_vessels(override_vessels) {
        println!("mod {} {}", vessel.ship_type, vessel.key);
    }

    Ok(())
}

fn read_vessels(path: impl AsRef<Path>) -> impl Iterator<Item = Vessel> {
    walkdir::WalkDir::new(path.as_ref())
        .into_iter()
        .filter_entry(|entry| {
            entry
                .metadata()
                .map(|data| data.file_type().is_file())
                .unwrap_or_default()
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == ".txt")
                    .unwrap_or_default()
        })
        .filter_map(|entry| load_vessel(entry.ok()?.path()).ok())
}

fn load_vessel(path: impl AsRef<Path>) -> Result<Vessel> {
    let name = path.as_ref().file_name().unwrap().to_string_lossy();
    let content = fs::read(&path)?;
    let content = String::from_utf8_lossy(&content);
    let vessel: VesselData = ini::from_str(&content)?;
    vessel.with_key(name)
}
