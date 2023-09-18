use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub gained: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Player {
    username: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Players {
    player: Player,
    pub data: Data,
}

pub const BOSSES: [&str; 58] = ["abyssal_sire", "alchemical_hydra", "artio", "barrows_chests", "bryophyta", "callisto", "calvarion", "cerberus", "chambers_of_xeric", "chambers_of_xeric_challenge_mode", "chaos_elemental", "chaos_fanatic", "commander_zilyana", "corporeal_beast", "crazy_archaeologist", "dagannoth_prime", "dagannoth_rex", "dagannoth_supreme", "deranged_archaeologist", "duke_sucellus", "general_graardor", "giant_mole", "grotesque_guardians", "hespori", "kalphite_queen", "king_black_dragon", "kraken", "kreearra", "kril_tsutsaroth", "mimic", "nex", "nightmare", "phosanis_nightmare", "obor", "phantom_muspah", "sarachnis", "scorpia", "skotizo", "spindel", "tempoross", "the_gauntlet", "the_corrupted_gauntlet", "the_leviathan", "the_whisperer", "theatre_of_blood", "theatre_of_blood_hard_mode", "thermonuclear_smoke_devil", "tombs_of_amascut", "tombs_of_amascut_expert", "tzkal_zuk", "tztok_jad", "vardorvis", "venenatis", "vetion", "vorkath", "wintertodt", "zalcano", "zulrah"];

impl Players {
    pub async fn get_totals(url: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let mut total: u32 = 0;
        let res: Vec<Players> = reqwest::get(url)
            .await?
            .json()
            .await?;

        for gain in 0..res.len() {
            total += res[gain].data.gained;
        }

        Ok(total)
    }
}
