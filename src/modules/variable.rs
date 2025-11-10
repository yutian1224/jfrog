use lazy_static::lazy_static;
use std::env;

pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref ACCESS_TOKEN: String = env::var("J_ACCESS_TOKEN").unwrap_or_default();
    pub static ref JFORG_URL: String = env::var("J_URL").unwrap_or_default();
    pub static ref JFROG_TOKEN: String = env::var("J_ENCRYPTED_TOKEN").unwrap_or_default();
    pub static ref BLOCK_GROUPS: Vec<String> = {
        let _s = env::var("J_BLOCK_GROUPS").unwrap_or_default();
        let mut _g = _s
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_lowercase().to_string())
            .collect::<Vec<String>>();
        _g.push("administrator".to_string());
        _g
    };
    pub static ref WHITE_GROUPS: Vec<String> = {
        let _s = env::var("J_WHITE_GROUPS").unwrap_or_default();
        let _g = _s
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_lowercase().to_string())
            .collect::<Vec<String>>();
        _g
    };
}
