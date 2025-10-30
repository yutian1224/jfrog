pub mod crypt;
pub mod group;
pub mod user;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use moka::future::Cache;
use tokio::time::Duration;

lazy_static! {
    pub static ref GROUP_CACHE: Cache<String, HashMap<String, String>> = Cache::builder()
        .time_to_live(Duration::from_secs(20))
        .build();
    pub static ref USER_CACHE: Cache<String, Vec<String>> = Cache::builder()
        .time_to_live(Duration::from_secs(5))
        .build();
}
