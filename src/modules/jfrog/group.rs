use crate::modules::{client::HTTP_CLIENT, variable::JFORG_URL};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::error;
use sonic_rs::{JsonContainerTrait, JsonValueTrait, Value};

lazy_static! {
    static ref URL: String = format!("{}{PATH_GROUPS}", *JFORG_URL);
}

static PATH_GROUPS: &str = "/access/api/v2/groups";

pub async fn groups_list() -> Result<HashMap<String, String>, String> {
    let mut resp = HashMap::new();
    let res = HTTP_CLIENT.get(&*URL).send().await;
    if let Err(e) = res {
        error!("get {:?} failed: {e}", *URL);
        return Err(e.to_string());
    }
    if let Ok(_res) = res {
        if !_res.status().is_success() {
            let txt = _res.text().await.unwrap_or_default();
            error!("get {:?} failed: {:?}", *URL, txt);
            return Err(txt);
        }
        let txt = _res.text().await.unwrap_or_default();
        let v: Value = sonic_rs::from_str(&txt).unwrap_or_default();
        v.get("groups").as_array().map(|groups| {
            groups.iter().for_each(|g| {
                if let Some(_gn) = g.get("group_name").as_str()
                    && let Some(_uri) = g.get("uri").as_str()
                {
                    resp.insert(_gn.to_string(), _uri.to_string());
                };
            });
            Some(())
        });
    }
    Ok(resp)
}
