mod cw;
mod error;
mod ini;

use cw::{Vessel, VesselData};
use hashbrown::HashMap;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, StructOpt)]
struct Paths {
    default_path: String,
    override_path: String,
}

#[derive(Clone, Debug, StructOpt)]
enum Opt {
    Missions(Paths),
    Traffic(Paths),
}

impl Opt {
    fn into_paths(self) -> (String, String) {
        match self {
            Opt::Missions(paths) | Opt::Traffic(paths) => {
                let Paths {
                    default_path,
                    override_path,
                } = paths;
                (default_path, override_path)
            }
        }
    }
}

fn main() -> Result<()> {
    use cw::MissionProfileData;

    let content = include_str!("../resource/the-duel.ini");
    let mission: MissionProfileData = ini::from_str(content)?;
    let mission = mission.into_profile()?;

    println!("{:#?}", mission);

    let (_default_path, override_path) = Opt::from_args().into_paths();

    let vessels: Vec<_> = cw::default_vessels().collect();
    let override_vessels: Vec<_> = read_vessels(override_path + "/vessels").collect();
    let mut vessels_by_key: HashMap<_, _> =
        vessels.iter().map(|vessel| (&vessel.key, vessel)).collect();

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
