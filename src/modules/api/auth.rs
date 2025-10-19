use crate::modules::{error::ApiError, variable::ACCESS_TOKEN};
use actix_web::{
    Error,
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};

pub struct CheckAuth;

impl<S, B> Transform<S, ServiceRequest> for CheckAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAuthMiddleware { service }))
    }
}
pub struct CheckAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        if !request.headers().contains_key("Authorization") {
            let (request, _pl) = request.into_parts();
            let response = ApiError::BadRequest("missing Authorization header")
                .to_response()
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let _auth = request
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .unwrap();
        if let Some(_token) = _auth.strip_prefix("Bearer ")
            && _token == *ACCESS_TOKEN
        {
            let res = self.service.call(request);
            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }
        let (request, _pl) = request.into_parts();
        let response = ApiError::BadRequest("authentication failed")
            .to_response()
            .map_into_right_body();
        Box::pin(async { Ok(ServiceResponse::new(request, response)) })
    }
}
