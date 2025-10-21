use crate::modules::{error::ApiError, jfrog::*};
use actix_web::{HttpResponse, web};
use hashbrown::HashMap;
use sonic_rs::{Deserialize, json};

#[derive(Debug, Deserialize)]
pub struct ApiUserGroupsGET {
    user: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiUserGroupsPOST {
    user: String,
    groups: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiUserGroupsAsPOST {
    user: String,
    other: String,
}

fn strip_email_suffix(email: &str) -> &str {
    match email.find('@') {
        Some(pos) => &email[..pos],
        None => email,
    }
}

pub async fn groups_get() -> HttpResponse {
    let gruops = group::groups_list().await;
    if let Err(e) = gruops {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let res: Vec<HashMap<String, String>> = gruops
        .unwrap_or_default()
        .keys()
        .filter(|g| *g != "administrator")
        .cloned()
        .map(|g| {
            let mut m = HashMap::new();
            m.insert("group".to_string(), g);
            m
        })
        .collect();
    let resp = json!(
        {
            "code": 200,
            "data": res,
            "result": true,
            "msg": ""
        }
    );
    HttpResponse::Ok().json(resp)
}

pub async fn user_groups_get(querys: web::Query<ApiUserGroupsGET>) -> HttpResponse {
    let user = strip_email_suffix(&querys.user);
    let groups = user::user_groups_list(user).await;
    if let Err(e) = groups {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let res: Vec<HashMap<String, String>> = groups
        .unwrap_or_default()
        .iter()
        .map(|g| {
            let mut m = HashMap::new();
            m.insert("group".to_string(), g.to_string());
            m
        })
        .collect();
    let resp = json!(
        {
            "code": 200,
            "data": res,
            "result": true,
            "msg": ""
        }
    );
    HttpResponse::Ok().json(resp)
}

pub async fn user_groups_post(data: web::Json<ApiUserGroupsPOST>) -> HttpResponse {
    let groups: Vec<String> = data
        .groups
        .iter()
        .flat_map(|g| {
            let s = g.trim_matches(|c: char| c == '(' || c == ')' || c.is_whitespace());
            s.split(',').filter_map(|part| {
                let p = part.trim().trim_matches('\'');
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                }
            })
        })
        .collect();
    let change_res = user::user_groups_change(strip_email_suffix(&data.user), groups).await;
    if let Err(e) = change_res {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let resp = json!(
        {
            "code": 200,
            "msg": "user groups updated successfully".to_string(),
            "result": true,
            "data": {}
        }
    );
    HttpResponse::Ok().json(resp)
}

pub async fn user_groups_add_post(data: web::Json<ApiUserGroupsPOST>) -> HttpResponse {
    let groups: Vec<String> = data
        .groups
        .iter()
        .flat_map(|g| {
            let s = g.trim_matches(|c: char| c == '(' || c == ')' || c.is_whitespace());
            s.split(',').filter_map(|part| {
                let p = part.trim().trim_matches('\'');
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                }
            })
        })
        .collect();
    let change_res = user::user_groups_add(strip_email_suffix(&data.user), groups).await;
    if let Err(e) = change_res {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let resp = json!(
        {
            "code": 200,
            "msg": "user groups added successfully".to_string(),
            "result": true,
            "data": {}
        }
    );
    HttpResponse::Ok().json(resp)
}

pub async fn user_groups_del_post(data: web::Json<ApiUserGroupsPOST>) -> HttpResponse {
    let groups: Vec<String> = data
        .groups
        .iter()
        .flat_map(|g| {
            let s = g.trim_matches(|c: char| c == '(' || c == ')' || c.is_whitespace());
            s.split(',').filter_map(|part| {
                let p = part.trim().trim_matches('\'');
                if p.is_empty() {
                    None
                } else {
                    Some(p.to_string())
                }
            })
        })
        .collect();
    let change_res = user::user_groups_del(strip_email_suffix(&data.user), groups).await;
    if let Err(e) = change_res {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let resp = json!(
        {
            "code": 200,
            "msg": "user groups deleted successfully".to_string(),
            "result": true,
            "data": {}
        }
    );
    HttpResponse::Ok().json(resp)
}

pub async fn user_groups_as_post(data: web::Json<ApiUserGroupsAsPOST>) -> HttpResponse {
    let change_res = user::user_groups_as_other(
        strip_email_suffix(&data.user),
        strip_email_suffix(&data.other),
    )
    .await;
    if let Err(e) = change_res {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let resp = json!(
        {
            "code": 200,
            "msg": "user groups updated successfully".to_string(),
            "result": true,
            "data": {}
        }
    );
    HttpResponse::Ok().json(resp)
}
