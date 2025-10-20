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
    let gruops = group::list_all_groups().await;
    if let Err(e) = gruops {
        return ApiError::ServiceUnavailable(&e).to_response();
    }
    let res: Vec<HashMap<String, String>> = gruops
        .unwrap_or_default()
        .iter()
        .map(|(g, _)| {
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

pub async fn user_groups_get(querys: web::Query<ApiUserGroupsGET>) -> HttpResponse {
    let user = strip_email_suffix(&querys.user);
    let groups = user::list_user_groups(user).await;
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
    let change_res =
        user::change_user_groups(strip_email_suffix(&data.user), data.groups.clone()).await;
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

pub async fn user_groups_as_post(data: web::Json<ApiUserGroupsAsPOST>) -> HttpResponse {
    let change_res = user::change_user_groups_as_other(
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
