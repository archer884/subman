use crate::cw::{CampaignRegion, ShipType, Vessel};

// This registry provides vessel metadata for those vessels which shipped with the original game.
// It is NOT necessary to add mod vessels to this list. Because this polyfill registry exists, it
// is also not necessary to actually read the default vessels directory.
//
// Note that, generally, dates given here are the year a class was first commissioned. Submarines
// such as Sturgeon, which received significant mid-life upgrades, are given two dates: the
// original commission date and the date of their second campaign.
fn default_vessels() -> impl Iterator<Item = Vessel> {
    static SOURCE_DATA: &[(&str, u16, ShipType, &[CampaignRegion])] = &[
        ("civ_fv_trawler", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_act_1", 1980, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_encounter", 1980, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_freighter_a", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_freighter_b", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_kommunist", 1980, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_leninskiy_komsomol", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_medium_tanker", 1990, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_mercur", 1990, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_old_small_freighter", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_poltava", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_qiongsha", 1990, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_roro", 1990, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_sl7", 1980, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_small_freighter", 1980, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_ms_super_p", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_oilrig", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_spar_rig", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_vtr_amguema", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_vtr_andizhan", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("civ_vtr_yuniy_partizan", 1980, ShipType::Submarine, &[CampaignRegion::All]),
        ("plan_aor_fuqing", 1980, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ap_qiongsha", 1980, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_as_daijiang", 1976, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ddg_luda", 1973, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ddg_luda3", 1991, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ddg_luhu", 1994, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ddg_sovremenny", 1999, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        // The fact that the Chengdu, Jianghu, and Jianghu iii classes have such different-looking
        // names may lull the Western observer into thinking these are different ships. They are
        // not. They are, according to the internet, all variants of the Riga, aka the Type 053
        // frigate in Chinese service. No wonder they're impossible to tell apart in game. -.-
        ("plan_ff_chengdu", 1974, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ff_jianghu", 1975, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ff_jianghu3", 1986, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ffg_jiangwei", 1998, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_lst_yukan", 1978, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_lst_yuting", 1990, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ptg_huangfeng", 1968, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ss_kilo", 1994, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ss_ming", 1962, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ss_romeo", 1962, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ssbn_xia", 1987, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ssk_song", 1998, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("plan_ssn_han", 1974, ShipType::Submarine, &[CampaignRegion::ChinaSeaNato]),
        ("usn_cg_belknap", 1964, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        // Historically, the Knox class were in commission from 1969. Again, pushing the date to
        // improve variety.
        ("usn_ff_knox", 1968, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_los_angeles", 1976, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_los_angeles_flt2", 1985, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_los_angeles_flt3", 1988, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        // Historically, USS Narwhal was not commissioned until 1969, but I've bumped up this
        // date so that Narwhal is eligible for inclusion in the 1968 Pact campaigns. Should
        // help to give them a little more variety.
        ("usn_ssn_narwhal", 1968, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        // Damn the depth!
        // Damn the pressure!
        // Take 'er down!
        // Down to Thresher!
        ("usn_ssn_permit", 1984, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_permit_68", 1961, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_seawolf", 1997, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_skipjack", 1984, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_skipjack_68", 1959, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_sturgeon", 1984, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("usn_ssn_sturgeon_68", 1967, ShipType::Submarine, &[CampaignRegion::ChinaSeaPlan, CampaignRegion::NorwegianSeaPact]),
        ("wp_bdk_alligator", 1964, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_bdk_ropucha", 1975, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_bpk_kanin", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_bpk_kara", 1971, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_bpk_kashin", 1962, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_bpk_kresta2", 1969, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_bpk_udaloy", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_em_kotlin", 1955, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_em_sovremenny", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_fv_trawler", 1960, ShipType::Submarine, &[CampaignRegion::All]),
        ("wp_kr_sverdlov", 1952, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_mpk_grisha3", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_mpk_poti", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ms_kommunist", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ms_leninskiy_komsomol", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ms_mercur", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ms_poltava", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ms_roro", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_pb_don", 1961, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_pb_ugra", 1963, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_pkr_moskva", 1984, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_pkr_moskva_68", 1967, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_rkr_kirov", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_rkr_kresta1", 1967, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_skr_krivak1", 1970, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_skr_riga", 1954, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ss_foxtrot", 1958, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ss_kilo", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        // No one seems to have any idea when the first Romeo was taken into Soviet service.
        ("wp_ss_romeo", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ss_tango", 1973, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ss_whiskey", 1950, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssb_golf", 1958, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssbn_delta3", 1976, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssbn_delta4", 1984, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssbn_typhoon", 1981, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssbn_yankee", 1967, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        // The key for this sub misspells the reporting name "Juliett."
        ("wp_ssg_juliet", 1962, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssgn_charlie1", 1967, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssgn_charlie2", 1973, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssgn_echo2", 1963, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssgn_oscar", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssgn_yasen", 2013, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_akula1", 1984, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_alfa", 1971, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_november", 1958, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_sierra", 1984, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_victor1", 1967, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_victor2", 1972, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_ssn_victor3", 1979, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_takr_kiev", 1975, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vt_boris_chilikin", 1970, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vt_dubna", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vt_kazbek", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vtr_amguema", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vtr_andizhan", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vtr_andizhan_mod", 1960, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
        ("wp_vtr_yuniy_partizan", 1980, ShipType::Submarine, &[CampaignRegion::NorwegianSeaNato, CampaignRegion::ChinaSeaNato, CampaignRegion::NorwegianSeaBrit]),
    ];

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

#[cfg(test)]
mod polyfill {
    #[test]
    fn all_vessels_have_non_zero_years() {
        for vessel in super::default_vessels() {
            assert!(vessel.year > 0, "Vessel has 0 as year: {}", vessel.key);
        }
    }
}