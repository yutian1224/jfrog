use lazy_static::lazy_static;
use std::env;

pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref ACCESS_TOKEN: String = env::var("J_ACCESS_TOKEN").unwrap_or_default();
    pub static ref JFORG_URL: String = env::var("J_URL")
        .unwrap_or("https://jfrog.devtest.infra.ww5sawfyut0k.bitsvc.io".to_string());
    pub static ref JFROG_TOKEN: String = env::var("J_ENCRYPTED_TOKEN").unwrap_or_default();
}
