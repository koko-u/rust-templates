use std::net;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use actix_web_httpauth::middleware::HttpAuthentication;
use natorus_web::app_cors;
use natorus_web::bearer_validator;
use natorus_web::errors::AppResult;
use natorus_web::errors::ToAppResult as _;
use natorus_web::routes::app_route;
use natorus_web::AppState;
use natorus_web::Db;
use natorus_web::EnvArgs;
use natorus_web::Jwt;

#[actix_web::main]
async fn main() -> AppResult<()> {
    let env_args = EnvArgs::init()?;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(&env_args.rust_log));

    let addrs = net::SocketAddr::from((env_args.host, env_args.port));

    let db = Db::init(&env_args).await?;
    let jwt = Jwt::new(&env_args);
    let data = web::Data::new(AppState { db, jwt });

    HttpServer::new(move || {
        let app_cors = app_cors(&env_args);
        let auth = HttpAuthentication::with_fn(bearer_validator::<Db>);

        App::new()
            .wrap(app_cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(app_route(data.clone(), auth))
    })
        .bind(addrs)
        .to_result()?
        .run()
        .await
        .to_result()?;

    Ok(())
}
