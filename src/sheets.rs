use google_sheets4::{api::ValueRange, api::UpdateValuesResponse ,hyper, hyper_rustls, Error, Sheets};

use crate::configs::Config;

pub async fn read(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {

    hub
        .spreadsheets()
        .values_get(&config.sheet_id, &config.output_range)
        .doit()
        .await
}

pub async fn write(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
    vals: ValueRange
) -> Result<(hyper::Response<hyper::Body>, UpdateValuesResponse), Error> {

    hub
        .spreadsheets()
        .values_update(vals, &config.sheet_id, &config.input_range)
        .value_input_option("RAW")
        .include_values_in_response(true)
        .doit()
        .await

}