use crate::cw::{CampaignRegion, ShipType, Vessel};

// This registry provides vessel metadata for those vessels which shipped with the original game.
// It is NOT necessary to add mod vessels to this list. Because this polyfill registry exists, it
// is also not necessary to actually read the default vessels directory.
fn default_vessels() -> impl Iterator<Item = Vessel> {
    static SOURCE_DATA: &[(&str, u16, ShipType, &[CampaignRegion])] = &[];

    SOURCE_DATA.into_iter().map(|data| {
        let &(key, year, ship_type, regions) = data;
        Vessel {
            key: key.into(),
            year,
            ship_type,
            op_force_regions: regions.iter().cloned().collect(),
        }
    })
}
