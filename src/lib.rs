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
    ) -> Result<Vec<(&'static str, u32)>, Box<dyn std::error::Error>> {
        let mut overall_total: u32 = 0;
        let mut url: String;
        let mut boss_total: u32;
        let mut totals = Vec::<(&'static str, u32)>::new();

        for (key, _) in bosses {
            url = format!("{}{}&period=week", base_url, key);
            boss_total = Self::get_boss_totals(&url).await?;
            totals.push((*key, boss_total));
            overall_total += boss_total;
            println!("{:?} = {}", key, boss_total);
        }
        println!("Overall: {}", overall_total);

        Ok(totals)
    }
}
