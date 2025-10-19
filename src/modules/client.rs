use crate::modules::{jfrog::crypt::decrypt, variable::JFROG_TOKEN};
use lazy_static::lazy_static;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = {
        if let Some(token) = decrypt(&JFROG_TOKEN) {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
            reqwest::Client::builder()
                .default_headers(headers)
                .timeout(tokio::time::Duration::from_secs(10))
                .tcp_keepalive(Some(tokio::time::Duration::from_secs(120)))
                .danger_accept_invalid_certs(true)
                .no_proxy()
                .build()
                .unwrap_or_default()
        } else {
            reqwest::Client::builder()
                .timeout(tokio::time::Duration::from_secs(10))
                .tcp_keepalive(Some(tokio::time::Duration::from_secs(120)))
                .danger_accept_invalid_certs(true)
                .no_proxy()
                .build()
                .unwrap_or_default()
        }
    };
}
