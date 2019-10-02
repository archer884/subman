mod ini;

use serde::Deserialize;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    /// Path of the game's default data directory
    default_path: String,

    /// Path of the mod's override directory
    override_path: String,
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Vessel {
    ship_type: String,
    year: String,
    player_fleet_regions: String,
    enemy_fleet_regions: String,
}

fn main() -> Result<()> {
    // use std::fs;

    // let Opt {
    //     default_path,
    //     override_path,
    // } = Opt::from_args();

    // let default_vessels = default_path + "/vessels";
    // let override_vessels = override_path + "/vessels";

    // for path in fs::read_dir(default_vessels)? {
    //     println!("Default: {}", path?.path().display());
    // }

    // for path in fs::read_dir(override_vessels)? {
    //     println!("Override: {}", path?.path().display());
    // }

    let ini_text = include_str!("../resource/the-duel.ini");
    let profile: IniMissionProfile = ini::from_str(ini_text)?;
    println!("{:#?}", profile);

    let ini_text = include_str!("../resource/usn_ssn_triton.txt");
    let vessel: Vessel = ini::from_str(ini_text)?;
    println!("{:#?}", vessel);

    Ok(())
}
