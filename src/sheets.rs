use google_sheets4::{
    api::UpdateValuesResponse, api::ValueRange, hyper, hyper_rustls, Error, Sheets,
};

use crate::configs::Config;

#[allow(dead_code)]
pub async fn read(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
    output_range: &str,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
    hub.spreadsheets()
        .values_get(&config.sheet_id, output_range)
        .doit()
        .await
}

pub async fn write(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
    cell_range: &String,
    vals: serde_json::Value,
) -> Result<(hyper::Response<hyper::Body>, UpdateValuesResponse), Error> {
    hub.spreadsheets()
        .values_update(
            ValueRange {
                major_dimension: Some(String::from("ROWS")),
                range: Some(cell_range.clone()),
                values: Some(vec![vec![vals]]),
            },
            &config.sheet_id,
            &cell_range
        )
        .value_input_option("RAW")
        .include_values_in_response(true)
        .doit()
        .await
}
