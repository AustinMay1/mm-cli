use dotenv::dotenv;
use std::env;

pub struct Config {
    pub priv_key: String,
    pub sheet_id: String,
    pub input_range: String,
    pub output_range: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        Config {
            priv_key: String::from("rusty_creds.json"),
            sheet_id: String::from(env::var("SPREADSHEET_ID").expect("No spreadsheet id provided.")),
            input_range: String::from("Sheet1!A1:C10"),
            output_range: String::from("Sheet1!A1:C10"),
        }
    }
}