use google_sheets4::oauth2::{self, authenticator::Authenticator};
use google_sheets4::{hyper, hyper_rustls};

use crate::configs::Config;

pub async fn auth(
    config: &Config,
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let secret: oauth2::ServiceAccountKey = oauth2::read_service_account_key(&config.priv_key)
        .await
        .expect("secret not found");

    oauth2::ServiceAccountAuthenticator::with_client(secret, client)
        .build()
        .await
        .expect("could not create an authenticator")
}
