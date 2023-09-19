#[allow(unused_imports)]
use monstermash::Players;
use google_sheets4::{api::ValueRange, Sheets};

mod auth;
mod configs;
mod sheets;
mod client;

const _BOSSES: [&str; 58] = [
    "abyssal_sire",
    "alchemical_hydra",
    "artio",
    "barrows_chests",
    "bryophyta",
    "callisto",
    "calvarion",
    "cerberus",
    "chambers_of_xeric",
    "chambers_of_xeric_challenge_mode",
    "chaos_elemental",
    "chaos_fanatic",
    "commander_zilyana",
    "corporeal_beast",
    "crazy_archaeologist",
    "dagannoth_prime",
    "dagannoth_rex",
    "dagannoth_supreme",
    "deranged_archaeologist",
    "duke_sucellus",
    "general_graardor",
    "giant_mole",
    "grotesque_guardians",
    "hespori",
    "kalphite_queen",
    "king_black_dragon",
    "kraken",
    "kreearra",
    "kril_tsutsaroth",
    "mimic",
    "nex",
    "nightmare",
    "phosanis_nightmare",
    "obor",
    "phantom_muspah",
    "sarachnis",
    "scorpia",
    "skotizo",
    "spindel",
    "tempoross",
    "the_gauntlet",
    "the_corrupted_gauntlet",
    "the_leviathan",
    "the_whisperer",
    "theatre_of_blood",
    "theatre_of_blood_hard_mode",
    "thermonuclear_smoke_devil",
    "tombs_of_amascut",
    "tombs_of_amascut_expert",
    "tzkal_zuk",
    "tztok_jad",
    "vardorvis",
    "venenatis",
    "vetion",
    "vorkath",
    "wintertodt",
    "zalcano",
    "zulrah",
];
const _BASE_URL: &str = "https://api.wiseoldman.net/v2/groups/1500/gained?metric=";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let total = Players::get_totals(BASE_URL, &BOSSES).await?;
    //println!("{:#?}", total);

    let config = configs::Config::new();
    let client = client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);

    let req = sheets::read(&hub, &config).await;

    match req {
        Err(e) => println!("{}", e),
        Ok((_, spreadsheet)) => {
            println!("Success: {:?}", 
                spreadsheet.values.unwrap()
            )
        }
    }
    
    let req = sheets::write(
        &hub, 
        &config, 
        ValueRange { 
            major_dimension: (Some(String::from("ROWS"))), 
            range: (Some(config.input_range.clone())), 
            values: Some(vec![vec![serde_json::Value::String("hello, world!".to_string())]])
        }).await;

    match req {
        Err(e) => println!("{}", e),
        Ok((_, spreadsheet)) => {
            println!("Success: {:?}",
                spreadsheet.updated_data.unwrap()
            )
        }
    }

    Ok(())
}
