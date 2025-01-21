use std::{env, net::TcpListener, sync::Arc};
use actix_web::{dev::Server, http::header, middleware::Logger, web::{self, ServiceConfig}, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use log::info;

use crate::api;

mod databases;
pub mod utils;
pub mod repositories;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

fn check_health(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/check_health").route(web::get().to(health)));
}

fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
}

pub async fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let _ = env_logger::try_init();

    // let pool = Arc::new(databases::postgresql::db_pool(db_name).await);

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || { 
        App::new()
        .wrap(cors())
            .wrap(Logger::default())
            // .app_data(pool.clone())
            .configure(check_health)
            .configure(api::routes)
        })
        .listen(listener)?
        .run();

    info!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}