mod ini;

use serde::Deserialize;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

// Your scientists... Seriously, though, why the fuck did you write an INI reader?

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
    let ini_text = include_str!("../resource/the-duel.ini");
    let profile: IniMissionProfile = ini::from_str(ini_text)?;

    println!("{:#?}", profile);

    Ok(())
}
