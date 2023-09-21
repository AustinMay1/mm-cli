use crate::constants::constants::*;
use google_sheets4::{api::ValueRange, Sheets};
use monstermash::Players;

mod auth;
mod client;
mod configs;
mod constants;
mod sheets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total = Players::get_totals(BASE_URL, &BOSSES).await?;

    let config = configs::Config::new();
    let client = client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);

    for (boss, cell) in BOSSES {
        for (key, val) in &total {
            if &boss == key {
                let cell_range = format!("Sheet1!{}", cell);

                let req = sheets::write(
                    &hub,
                    &config,
                    &cell_range,
                    ValueRange {
                        major_dimension: Some(String::from("ROWS")),
                        range: Some(cell_range.clone()),
                        values: Some(vec![vec![serde_json::Value::Number(val.clone().into())]]),
                    },
                )
                .await;

                match req {
                    Err(e) => println!("{}", e),
                    Ok((_, spreadsheet)) => {
                        println!("Success: {:?}", spreadsheet.updated_data.unwrap())
                    }
                }
            }
        }
    }

    Ok(())
}
