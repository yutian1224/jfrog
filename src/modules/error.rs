use actix_web::HttpResponse;
use faststr::FastStr;
use sonic_rs::Serialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError<'a> {
    #[error("{0}")]
    BadRequest(&'a str),
    #[error("{0}")]
    ServiceUnavailable(&'a str),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    msg: FastStr,
}

impl ApiError<'_> {
    pub fn to_response(&self) -> HttpResponse {
        let (status, msg) = match self {
            ApiError::BadRequest(msg) => (400, FastStr::from_str(msg).unwrap_or_default()),
            ApiError::ServiceUnavailable(msg) => (503, FastStr::from_str(msg).unwrap_or_default()),
        };

        let response = ErrorResponse { code: status, msg };

        match status {
            304 => HttpResponse::NotModified().json(response),
            400 => HttpResponse::BadRequest().json(response),
            503 => HttpResponse::ServiceUnavailable().json(response),
            _ => HttpResponse::InternalServerError().json(response),
        }
    }
}
