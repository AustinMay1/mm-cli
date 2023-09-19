use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    async fn get_boss_totals(url: String) -> Result<u32, Box<dyn std::error::Error>> {
        let mut total: u32 = 0;
        let players: Vec<Self> = reqwest::get(url)
            .await?
            .json()
            .await?;

        for player in players.iter() {
            total += player.data.gained;
        }

        Ok(total)
    }

    pub async fn get_totals(base_url: &str, bosses: &[&'static str]) -> Result<HashMap<&'static str, u32>, Box<dyn std::error::Error>> {
        let mut overall_total: u32 = 0;
        let mut url: String;
        let mut boss_total: u32;
        let mut totals = HashMap::new();

        for boss in bosses.iter() {
            url = String::from(base_url.to_owned() + boss + "&period=week");
            boss_total = Self::get_boss_totals(url).await?;
            totals.insert(*boss, boss_total);
            overall_total += boss_total;
            println!("{:?} = {}", boss, boss_total);
        }
        println!("Overall: {}", overall_total);

        Ok(totals)
    }
}
