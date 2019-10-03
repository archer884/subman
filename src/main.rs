mod cw;
mod error;
mod ini;

use cw::{Vessel, VesselData};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use hashbrown::HashMap;

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    /// Path of the game's default data directory
    default_path: String,

    /// Path of the mod's override directory
    override_path: String,
}

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
        override_path,
        ..
    } = Opt::from_args();

    let vessels: Vec<_> = cw::default_vessels().collect();
    let override_vessels: Vec<_> = read_vessels(override_path + "/vessels").collect();
    let mut vessels_by_key: HashMap<_, _> = vessels.iter().map(|vessel| (&vessel.key, vessel)).collect();

    override_vessels.iter().for_each(|vessel| {
        vessels_by_key
            .entry(&vessel.key)
            .and_modify(|x| *x = vessel)
            .or_insert(vessel);
    });

    for (key, vessel) in &vessels_by_key {
        println!("{} {} {:?}", vessel.year, key, vessel.op_force_regions);
    }

    Ok(())
}

fn read_vessels(path: impl AsRef<Path>) -> impl Iterator<Item = Vessel> {
    fn is_text_file(path: &Path) -> bool {
        path.extension().map(|x| x == "txt").unwrap_or_default()
    }
    
    walkdir::WalkDir::new(path.as_ref())
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let meta = entry.metadata().ok()?;
            if meta.file_type().is_file() && is_text_file(entry.path()) {
                load_vessel(entry.path()).ok()
            } else {
                None
            }
        })
}

fn load_vessel(path: impl AsRef<Path>) -> Result<Vessel> {
    let name = path.as_ref().file_stem().unwrap().to_string_lossy();
    let content = fs::read(&path)?;
    let content = String::from_utf8_lossy(&content);
    let vessel: VesselData = ini::from_str(&content)?;
    vessel.with_key(name)
}
