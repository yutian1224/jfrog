use crate::modules::{client::HTTP_CLIENT, variable::JFORG_URL};
use lazy_static::lazy_static;
use log::{error, info, warn};
use sonic_rs::{JsonContainerTrait, JsonValueTrait, Value, json};

lazy_static! {
    static ref URL: String = format!("{}{PATH_USERS}", *JFORG_URL);
}

static PATH_USERS: &str = "/access/api/v2/users";

pub async fn user_groups_list(user: &str) -> Result<Vec<String>, String> {
    let url = format!("{}/{user}", *URL);
    let mut resp = Vec::new();
    let res = HTTP_CLIENT.get(&url).send().await;
    if let Err(e) = res {
        warn!("[{user}]get user groups failed: {e}");
        return Err(e.to_string());
    }
    if let Ok(_res) = res {
        if !_res.status().is_success() {
            let txt = _res.text().await.unwrap_or_default();
            warn!("[{user}]get user groups failed: {txt}");
            return Err(txt);
        }
        let txt = _res.text().await.unwrap_or_default();
        let v: Value = sonic_rs::from_str(&txt).unwrap_or_default();
        if let Some(groups) = v.get("groups").as_array() {
            groups.iter().for_each(|g| {
                if let Some(_g) = g.as_str() {
                    resp.push(_g.to_string());
                }
            });
        }
    }
    info!("[{user}]current groups: {:?}", resp);
    Ok(resp)
}

pub async fn user_groups_add(user: &str, mut groups: Vec<String>) -> Result<(), String> {
    groups.retain(|g| g != "administrator");
    let url = format!("{}/{user}/groups", *URL);
    let recent_groups = user_groups_list(user).await?;
    info!("[{user}]adding groups: {:?}", groups);
    let to_add: Vec<String> = groups
        .iter()
        .filter(|g| !recent_groups.contains(g))
        .cloned()
        .collect();
    if to_add.is_empty() {
        info!("[{user}]no changes");
        return Ok(());
    }
    let body = json!({
        "add": to_add,
        "remove": [],
    });
    info!("[{user}]changes for patch: {}", body);
    let res = HTTP_CLIENT.patch(&url).json(&body).send().await;
    if let Err(e) = res {
        error!("[{user}]patch failed: {e}");
        return Err(e.to_string());
    }
    if let Ok(_res) = res
        && !_res.status().is_success()
    {
        let code = _res.status();
        let txt = _res.text().await.unwrap_or_default();
        error!("[{user}]patch failed: code {}, {:?}", code, txt);
        return Err(txt);
    }
    Ok(())
}

pub async fn user_groups_del(user: &str, mut groups: Vec<String>) -> Result<(), String> {
    groups.retain(|g| g != "administrator");
    let url = format!("{}/{user}/groups", *URL);
    let recent_groups = user_groups_list(user).await?;
    info!("[{user}]deleting groups: {:?}", groups);
    let to_remove: Vec<String> = recent_groups
        .iter()
        .filter(|g| groups.contains(g))
        .cloned()
        .collect();
    if to_remove.is_empty() {
        info!("[{user}]no changes");
        return Ok(());
    }
    let body = json!({
        "add": [],
        "remove": to_remove,
    });
    info!("[{user}]changes for patch: {}", body);
    let res = HTTP_CLIENT.patch(&url).json(&body).send().await;
    if let Err(e) = res {
        error!("[{user}]patch failed: {e}");
        return Err(e.to_string());
    }
    if let Ok(_res) = res
        && !_res.status().is_success()
    {
        let code = _res.status();
        let txt = _res.text().await.unwrap_or_default();
        error!("[{user}]patch failed: code {}, {:?}", code, txt);
        return Err(txt);
    }
    Ok(())
}

pub async fn user_groups_change(user: &str, mut groups: Vec<String>) -> Result<(), String> {
    groups.retain(|g| g != "administrator");
    let url = format!("{}/{user}/groups", *URL);
    let recent_groups = user_groups_list(user).await?;
    info!("[{user}]changing groups to: {:?}", groups);
    let to_add: Vec<String> = groups
        .iter()
        .filter(|g| !recent_groups.contains(g))
        .cloned()
        .collect();
    let to_remove: Vec<String> = recent_groups
        .iter()
        .filter(|g| !groups.contains(g))
        .cloned()
        .collect();
    if to_add.is_empty() && to_remove.is_empty() {
        info!("[{user}]no changes");
        return Ok(());
    }
    let body = json!({
        "add": to_add,
        "remove": to_remove,
    });
    info!("[{user}]changes for patch: {}", body);
    let res = HTTP_CLIENT.patch(&url).json(&body).send().await;
    if let Err(e) = res {
        error!("[{user}]patch failed: {e}");
        return Err(e.to_string());
    }
    if let Ok(_res) = res
        && !_res.status().is_success()
    {
        let code = _res.status();
        let txt = _res.text().await.unwrap_or_default();
        error!("[{user}]patch failed: code {}, {:?}", code, txt);
        return Err(txt);
    }
    Ok(())
}

pub async fn user_groups_as_other(user: &str, other: &str) -> Result<(), String> {
    let other_groups = user_groups_list(other).await?;
    user_groups_change(user, other_groups).await
}
