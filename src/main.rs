use monstermash::Players;

const BOSSES: [&str; 58] = ["abyssal_sire", "alchemical_hydra", "artio", "barrows_chests", "bryophyta", "callisto", "calvarion", "cerberus", "chambers_of_xeric", "chambers_of_xeric_challenge_mode", "chaos_elemental", "chaos_fanatic", "commander_zilyana", "corporeal_beast", "crazy_archaeologist", "dagannoth_prime", "dagannoth_rex", "dagannoth_supreme", "deranged_archaeologist", "duke_sucellus", "general_graardor", "giant_mole", "grotesque_guardians", "hespori", "kalphite_queen", "king_black_dragon", "kraken", "kreearra", "kril_tsutsaroth", "mimic", "nex", "nightmare", "phosanis_nightmare", "obor", "phantom_muspah", "sarachnis", "scorpia", "skotizo", "spindel", "tempoross", "the_gauntlet", "the_corrupted_gauntlet", "the_leviathan", "the_whisperer", "theatre_of_blood", "theatre_of_blood_hard_mode", "thermonuclear_smoke_devil", "tombs_of_amascut", "tombs_of_amascut_expert", "tzkal_zuk", "tztok_jad", "vardorvis", "venenatis", "vetion", "vorkath", "wintertodt", "zalcano", "zulrah"];
const BASE_URL: &str = "https://api.wiseoldman.net/v2/groups/1500/gained?metric="; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let total = Players::get_totals(BASE_URL, &BOSSES).await?;
    println!("{:#?}", total);
    Ok(())
}
