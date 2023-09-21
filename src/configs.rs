use dotenv::dotenv;
use std::env;

pub struct Config {
    pub priv_key: String,
    pub sheet_id: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        Config {
            priv_key: String::from("rusty_creds.json"),
            sheet_id: env::var("SPREADSHEET_ID").expect("No spreadsheet id provided."),
        }
    }
}
