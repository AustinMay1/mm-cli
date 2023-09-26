use crate::constants::consts::*;
use google_sheets4::Sheets;
use monstermash::{Players, process_args};
use serde_json::json;
use clap::Parser;

mod auth;
mod client;
mod configs;
mod constants;
mod sheets;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let date_from = process_args(&args.from).unwrap();
    let date_to = process_args(&args.to).unwrap();
    let total = Players::get_totals(BASE_URL, &BOSSES, &date_from, &date_to).await?;
    let config = configs::Config::new();
    let client = client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);

    println!("Date From: {:?}", date_from);
    println!("Date To: {:?}", date_to);

    for (boss, cell) in BOSSES {
        for (key, val) in &total {
            if &boss == key {
                let cell_range = format!("Sheet1!{}", cell);

                let req = sheets::write(
                    &hub,
                    &config,
                    &cell_range,
                    json!(val),
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
