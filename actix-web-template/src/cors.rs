use actix_cors::Cors;
use actix_web::http;

use crate::EnvArgs;

pub fn app_cors(env_args: &EnvArgs) -> Cors {
    let mut cors = Cors::default();
    for allowed_origin in env_args.allowed_origins.iter() {
        cors = cors.allowed_origin(allowed_origin);
    }

    cors.allowed_methods([
        http::Method::GET,
        http::Method::POST,
        http::Method::PUT,
        http::Method::PATCH,
        http::Method::DELETE,
        http::Method::HEAD,
    ])
        .allowed_headers([
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(time::Duration::seconds(30).as_seconds_f32() as usize)
}
