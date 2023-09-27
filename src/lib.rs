use chrono::prelude::*;
use serde::{Deserialize, Serialize};

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

impl Players {
    async fn get_boss_totals(url: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let mut total: u32 = 0;
        let players: Vec<Self> = reqwest::get(url).await?.json().await?;

        for player in players {
            total += player.data.gained;
        }

        Ok(total)
    }

    pub async fn get_totals(
        base_url: &str,
        bosses: &[(&'static str, &'static str)],
        date_from: &DateTime<chrono::Utc>,
        date_to: &DateTime<chrono::Utc>,
    ) -> Result<Vec<(&'static str, u32)>, Box<dyn std::error::Error>> {
        let mut overall_total: u32 = 0;
        let mut url: String;
        let mut boss_total: u32;
        let mut totals = Vec::<(&'static str, u32)>::new();

        for (key, _) in bosses {
            url = format!(
                "{}{}&startDate={:?}&endDate={:?}",
                base_url, key, date_from, date_to
            );
            boss_total = Self::get_boss_totals(&url).await?;
            totals.push((*key, boss_total));
            overall_total += boss_total;
            println!("{:?} = {}", key, boss_total);
        }
        println!("Overall: {}", overall_total);

        Ok(totals)
    }
}

pub fn process_args(arg: &String) -> Option<DateTime<chrono::Utc>> {
    if arg.len() < 7 || arg.len() > 7 {
        panic!("[ERROR]: Invalid from/to date. Please use format YYYYMMDD.")
    }

    let year: i32 = arg[0..=3].parse().unwrap();
    let month: u32 = arg[4..=5].parse().unwrap();
    let day: u32 = arg[6..=7].parse().unwrap();

    let dt = NaiveDate::from_ymd_opt(year, month, day)?
        .and_hms_milli_opt(0, 0, 0, 0)?
        .and_local_timezone(Utc)
        .unwrap();

    Some(dt)
}
