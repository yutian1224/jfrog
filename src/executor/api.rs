use super::init::{INIT_DONE as LOGGER_INIT_DONE, INIT_NOTIFY as LOGGER_INIT_NOTIFY};
use crate::modules::{
    api::{
        auth::CheckAuth,
        v1::{
            groups_get as v1_groups_get, user_groups_as_post as v1_user_groups_as_post,
            user_groups_get as v1_user_groups_get, user_groups_post as v1_user_groups_post,
        },
    },
    variable::PKG_VERSION,
};
use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use std::{sync::atomic::Ordering, time::Duration};

#[actix_web::main]
pub async fn init(addr: String, worker: usize) -> std::io::Result<()> {
    if !LOGGER_INIT_DONE.load(Ordering::Acquire) {
        LOGGER_INIT_NOTIFY.notified().await;
    }

    let app = move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("server", format!("jfrog_supervisor/{PKG_VERSION}"))),
            )
            .app_data(web::JsonConfig::default().limit(1_485_760))
            // .service(web::resource("/metrics").route(web::get().to(api_metrics)))
            .service(
                web::scope("/api")
                    .wrap(CheckAuth)
                    .wrap(
                        middleware::DefaultHeaders::new().add(("Content-Type", "application/json")),
                    )
                    .service(web::resource("/v1/groups").route(web::get().to(v1_groups_get)))
                    .service(
                        web::resource("/v1/user/groups")
                            .route(web::get().to(v1_user_groups_get))
                            .route(web::post().to(v1_user_groups_post)),
                    )
                    .service(
                        web::resource("/v1/user/groups/as")
                            .route(web::post().to(v1_user_groups_as_post)),
                    ),
            )
            .default_service(web::to(HttpResponse::NotFound))
    };
    HttpServer::new(app)
        .keep_alive(Duration::from_secs(60))
        .shutdown_timeout(10)
        .workers(worker)
        .bind(addr)?
        .run()
        .await
}
