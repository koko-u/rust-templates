use std::future;

use actix_web::dev;
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::AppState;

pub fn app_route<DB, F, O>(
    data: web::Data<AppState<DB>>,
    auth: HttpAuthentication<Option<BearerAuth>, F>,
) -> impl FnOnce(&mut web::ServiceConfig)
where
    DB: 'static,
    F: Fn(dev::ServiceRequest, Option<BearerAuth>) -> O + 'static,
    O: future::Future<
        Output = Result<dev::ServiceRequest, (actix_web::error::Error, dev::ServiceRequest)>,
    > + 'static,
{
    move |cfg| {
        cfg.app_data(data)
           .service(
               web::scope("/api/users")
                   .wrap(auth)
                   .route("", web::get().to(|| async { "GET /api/users" }))
                   .route("/{id}", web::get().to(|| async { "GET /api/users/{id}" })),
           )
           .service(
               web::scope("/api/accounts")
                   .route(
                       "/register",
                       web::post().to(|| async { "POST /api/accounts/register" }),
                   )
                   .route(
                       "/login",
                       web::post().to(|| async { "POST /api/accounts/login" }),
                   ),
           );
    }
}
